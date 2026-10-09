use crate::{error::ApiError, AppState};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Item {
    pub id: i64,
    pub uuid: String,
    pub company_uuid: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemBody {
    pub title: String,
    /// Em produção preferir company do token/introspect; header/body só para scaffold.
    #[serde(default)]
    pub company_uuid: Option<String>,
}

#[derive(Debug, Serialize)]
struct ListResponse {
    status: bool,
    rows: Vec<Item>,
}

fn resolve_company(
    auth_company: Option<String>,
    request: &HttpRequest,
) -> Result<String, ApiError> {
    if let Some(company) = auth_company.filter(|v| !v.trim().is_empty()) {
        return Ok(company);
    }
    if let Some(header) = request
        .headers()
        .get("x-company-uuid")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        return Ok(header.to_string());
    }
    Err(ApiError::BadRequest(
        "company_uuid ausente (token ou header X-Company-Uuid)".into(),
    ))
}

pub async fn list_items(
    state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let auth = state.auth.authenticate(&request).await?;
    let company_uuid = resolve_company(auth.company_uuid, &request)?;

    let rows = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, uuid, company_uuid, title, created_at, updated_at, deleted_at, deleted_by
          FROM items
         WHERE company_uuid = ? AND deleted_at IS NULL
         ORDER BY id DESC
         LIMIT 100
        "#,
    )
    .bind(&company_uuid)
    .fetch_all(&state.pool)
    .await
    .map_err(|error| {
        state.reporter.report(format!("list items: {error}"));
        ApiError::Internal(anyhow::Error::new(error).context("list items"))
    })?;

    Ok(HttpResponse::Ok().json(ListResponse { status: true, rows }))
}

pub async fn get_item(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let auth = state.auth.authenticate(&request).await?;
    let company_uuid = resolve_company(auth.company_uuid, &request)?;
    let item_uuid = path.into_inner();

    let row = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, uuid, company_uuid, title, created_at, updated_at, deleted_at, deleted_by
          FROM items
         WHERE uuid = ? AND company_uuid = ? AND deleted_at IS NULL
         LIMIT 1
        "#,
    )
    .bind(&item_uuid)
    .bind(&company_uuid)
    .fetch_optional(&state.pool)
    .await
    .map_err(|error| {
        state.reporter.report(format!("get item: {error}"));
        ApiError::Internal(anyhow::Error::new(error).context("get item"))
    })?;

    match row {
        Some(item) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": true,
            "row": item
        }))),
        None => Err(ApiError::NotFound),
    }
}

pub async fn create_item(
    state: web::Data<AppState>,
    request: HttpRequest,
    body: web::Json<CreateItemBody>,
) -> Result<HttpResponse, ApiError> {
    let auth = state.auth.authenticate(&request).await?;
    let company_uuid = resolve_company(
        auth.company_uuid.or_else(|| body.company_uuid.clone()),
        &request,
    )?;

    let title = body.title.trim();
    if title.is_empty() || title.chars().count() > 255 {
        return Err(ApiError::BadRequest(
            "title deve ter entre 1 e 255 caracteres".into(),
        ));
    }

    let item_uuid = Uuid::new_v4().to_string();
    let result = sqlx::query(
        r#"
        INSERT INTO items (uuid, company_uuid, title)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(&item_uuid)
    .bind(&company_uuid)
    .bind(title)
    .execute(&state.pool)
    .await
    .map_err(|error| {
        state.reporter.report(format!("create item: {error}"));
        ApiError::Internal(anyhow::Error::new(error).context("create item"))
    })?;

    let id = result.last_insert_id() as i64;
    let row = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, uuid, company_uuid, title, created_at, updated_at, deleted_at, deleted_by
          FROM items WHERE id = ? LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|error| ApiError::Internal(anyhow::Error::new(error).context("reload item")))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": true,
        "row": row
    })))
}

/// Soft delete — nunca DELETE físico por ação de usuário.
pub async fn soft_delete_item(
    state: web::Data<AppState>,
    request: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let auth = state.auth.authenticate(&request).await?;
    let company_uuid = resolve_company(auth.company_uuid, &request)?;
    let item_uuid = path.into_inner();
    let deleted_by = auth.user_uuid.unwrap_or_else(|| "unknown".into());

    let result = sqlx::query(
        r#"
        UPDATE items
           SET deleted_at = UTC_TIMESTAMP(),
               deleted_by = ?,
               updated_at = UTC_TIMESTAMP()
         WHERE uuid = ? AND company_uuid = ? AND deleted_at IS NULL
        "#,
    )
    .bind(&deleted_by)
    .bind(&item_uuid)
    .bind(&company_uuid)
    .execute(&state.pool)
    .await
    .map_err(|error| {
        state.reporter.report(format!("soft delete item: {error}"));
        ApiError::Internal(anyhow::Error::new(error).context("soft delete item"))
    })?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": true,
        "message": "deleted"
    })))
}
