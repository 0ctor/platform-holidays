use anyhow::{bail, Context, Result};
use std::{env, time::Duration};

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_address: String,
    pub database_url: String,
    pub database_max_connections: u32,
    pub auth_introspect_url: String,
    pub auth_timeout: Duration,
    pub allowed_cors_origins: Vec<String>,
    pub environment: String,
    pub service_name: String,
    pub loki: Option<LokiConfig>,
}

#[derive(Clone, Debug)]
pub struct LokiConfig {
    pub push_url: String,
    pub user: String,
    pub password: String,
    pub job: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let _ = dotenvy::dotenv();

        let database_url = env::var("DATABASE_URL").context("DATABASE_URL é obrigatório")?;
        if !database_url.starts_with("mysql://") {
            bail!("DATABASE_URL deve usar MySQL (mysql://…)");
        }

        let auth_introspect_url = env::var("AUTH_INTROSPECT_URL")
            .unwrap_or_else(|_| "https://auth.octor.com.br/v1/auth/introspect".into());
        let allowed_cors_origins = env::var("ALLOWED_CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".into())
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .collect();

        Ok(Self {
            bind_address: env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".into()),
            database_url,
            database_max_connections: parse_env("DATABASE_MAX_CONNECTIONS", 10)?,
            auth_introspect_url,
            auth_timeout: Duration::from_millis(parse_env("AUTH_TIMEOUT_MS", 2_000)?),
            allowed_cors_origins,
            environment: env::var("APP_ENV")
                .or_else(|_| env::var("ENVIRONMENT"))
                .unwrap_or_else(|_| "production".into()),
            service_name: env::var("OCTOR_SERVICE_NAME")
                .unwrap_or_else(|_| "template-api-rust".into()),
            loki: LokiConfig::from_env()?,
        })
    }
}

impl LokiConfig {
    fn from_env() -> Result<Option<Self>> {
        let Some(push_url) = optional_env("LOKI_PUSH_URL") else {
            return Ok(None);
        };

        Ok(Some(Self {
            push_url,
            user: env::var("LOKI_PUSH_USER")
                .context("LOKI_PUSH_USER é obrigatório quando LOKI_PUSH_URL estiver definido")?,
            password: env::var("LOKI_PUSH_PASSWORD").context(
                "LOKI_PUSH_PASSWORD é obrigatório quando LOKI_PUSH_URL estiver definido",
            )?,
            job: env::var("LOKI_JOB").unwrap_or_else(|_| "template-api-rust".into()),
        }))
    }
}

fn optional_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

fn parse_env<T>(name: &str, default: T) -> Result<T>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    match env::var(name) {
        Ok(value) => value.parse().with_context(|| format!("{name} inválido")),
        Err(_) => Ok(default),
    }
}
