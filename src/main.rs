use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use anyhow::{Context, Result};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;

mod auth;
mod config;
mod error;
mod handlers;
mod holidays;
mod observability;

use auth::Authenticator;
use config::Config;
use handlers::error_report::ErrorReportRateLimiter;
use observability::ErrorReporter;

#[derive(Clone)]
pub struct AppState {
    pub auth: Authenticator,
    pub reporter: ErrorReporter,
    pub service_name: String,
}

#[actix_web::main]
async fn main() -> Result<()> {
    init_tracing();
    let config = Config::from_env()?;

    let auth = Authenticator::new(config.auth_introspect_url.clone(), config.auth_timeout)
        .context("cliente de autenticação")?;
    let state = AppState {
        auth,
        reporter: ErrorReporter::new(config.loki.clone(), config.environment.clone()),
        service_name: config.service_name.clone(),
    };
    let limiter = web::Data::new(ErrorReportRateLimiter::default());
    let bind_address = config.bind_address.clone();
    let origins = config.allowed_cors_origins.clone();

    tracing::info!(%bind_address, service = %config.service_name, "iniciando API de feriados");
    HttpServer::new(move || {
        let allowed_origins = origins.clone();
        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _| {
                origin
                    .to_str()
                    .ok()
                    .is_some_and(|value| allowed_origins.iter().any(|item| item == value))
            })
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(limiter.clone())
            .wrap(cors)
            .wrap(TracingLogger::default())
            .route("/health/live", web::get().to(handlers::health::health_live))
            .route(
                "/health/ready",
                web::get().to(handlers::health::health_ready),
            )
            // Compatibilidade com Holidaysapi PHP (path estilo GitHub Pages)
            .route(
                "/health_calendar.json",
                web::get().to(handlers::holidays::legacy_health_calendar),
            )
            .route(
                "/national.json",
                web::get().to(handlers::holidays::national),
            )
            .service(
                web::scope("/v1")
                    .route("/health/live", web::get().to(handlers::health::health_live))
                    .route(
                        "/health/ready",
                        web::get().to(handlers::health::health_ready),
                    )
                    .route(
                        "/errors/report",
                        web::post().to(handlers::error_report::report_client_error),
                    )
                    .route("/holidays", web::get().to(handlers::holidays::list))
                    .route(
                        "/holidays/national",
                        web::get().to(handlers::holidays::national),
                    )
                    .route(
                        "/holidays/state/{uf}",
                        web::get().to(handlers::holidays::state),
                    )
                    .route(
                        "/holidays/city/{city_ibge}",
                        web::get().to(handlers::holidays::city),
                    )
                    .route(
                        "/health-calendar",
                        web::get().to(handlers::holidays::legacy_health_calendar),
                    ),
            )
    })
    .bind(&bind_address)
    .with_context(|| format!("bind em {bind_address}"))?
    .shutdown_timeout(10)
    .run()
    .await
    .context("servidor HTTP")
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("platform_holidays=info,actix_web=info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .json()
        .with_target(false)
        .init();
}
