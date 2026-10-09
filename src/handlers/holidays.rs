use crate::{
    error::ApiError,
    holidays::{list_holidays, Holiday, HolidayQuery, HolidayScope},
};
use actix_web::{web, HttpResponse};
use chrono::{Datelike, NaiveDate};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    /// ISO date YYYY-MM-DD (default: 1º de janeiro do ano corrente / `year`)
    pub from: Option<String>,
    /// ISO date YYYY-MM-DD (default: 31/12 do ano corrente / `year`)
    pub to: Option<String>,
    /// Atalho: ano civil completo
    pub year: Option<i32>,
    /// national,state,city,health (csv). Default: national
    pub scopes: Option<String>,
    pub uf: Option<String>,
    pub city_ibge: Option<String>,
}

fn parse_date(raw: &str, field: &str) -> Result<NaiveDate, ApiError> {
    NaiveDate::parse_from_str(raw, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest(format!("{field} deve ser YYYY-MM-DD")))
}

fn resolve_range(query: &ListQuery) -> Result<(NaiveDate, NaiveDate), ApiError> {
    if let Some(year) = query.year {
        if !(1583..=9999).contains(&year) {
            return Err(ApiError::BadRequest(
                "year fora do intervalo suportado".into(),
            ));
        }
        let from = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| ApiError::BadRequest("year inválido".into()))?;
        let to = NaiveDate::from_ymd_opt(year, 12, 31)
            .ok_or_else(|| ApiError::BadRequest("year inválido".into()))?;
        return Ok((from, to));
    }

    let today = chrono::Utc::now().date_naive();
    let from = match &query.from {
        Some(v) => parse_date(v, "from")?,
        None => NaiveDate::from_ymd_opt(today.year(), 1, 1).unwrap(),
    };
    let to = match &query.to {
        Some(v) => parse_date(v, "to")?,
        None => NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap(),
    };
    Ok((from, to))
}

pub async fn list(query: web::Query<ListQuery>) -> Result<HttpResponse, ApiError> {
    let (from, to) = resolve_range(&query)?;
    let scopes = match &query.scopes {
        Some(raw) => HolidayScope::parse_list(raw).map_err(ApiError::BadRequest)?,
        None => vec![HolidayScope::National],
    };

    let items = list_holidays(&HolidayQuery {
        from,
        to,
        scopes,
        uf: query.uf.clone(),
        city_ibge: query.city_ibge.clone(),
    })
    .map_err(ApiError::BadRequest)?;

    Ok(HttpResponse::Ok().json(json!({
        "status": true,
        "from": from,
        "to": to,
        "count": items.len(),
        "holidays": items,
    })))
}

pub async fn national(query: web::Query<ListQuery>) -> Result<HttpResponse, ApiError> {
    let mut q = query.into_inner();
    q.scopes = Some("national".into());
    list(web::Query(q)).await
}

pub async fn state(
    path: web::Path<String>,
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError> {
    let mut q = query.into_inner();
    q.scopes = Some("state".into());
    q.uf = Some(path.into_inner());
    list(web::Query(q)).await
}

pub async fn city(
    path: web::Path<String>,
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError> {
    let mut q = query.into_inner();
    q.scopes = Some("city".into());
    q.city_ibge = Some(path.into_inner());
    list(web::Query(q)).await
}

/// Formato legado compatível com `Holidaysapi::health_calendar()` do platform-legacy:
/// array de objetos com `date` = DD-MM, `title`, `description`, `category`, `categoryColor`, `legislation`.
/// Inclui feriados nacionais (datas móveis corretas no ano corrente) + calendário da saúde.
pub async fn legacy_health_calendar(
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError> {
    let year = query
        .year
        .unwrap_or_else(|| chrono::Utc::now().date_naive().year());
    let from = NaiveDate::from_ymd_opt(year, 1, 1)
        .ok_or_else(|| ApiError::BadRequest("year inválido".into()))?;
    let to = NaiveDate::from_ymd_opt(year, 12, 31)
        .ok_or_else(|| ApiError::BadRequest("year inválido".into()))?;

    let items = list_holidays(&HolidayQuery {
        from,
        to,
        scopes: vec![HolidayScope::National, HolidayScope::Health],
        uf: None,
        city_ibge: None,
    })
    .map_err(ApiError::BadRequest)?;

    let payload: Vec<Value> = items.iter().map(to_legacy_item).collect();
    Ok(HttpResponse::Ok().json(payload))
}

fn to_legacy_item(h: &Holiday) -> Value {
    let category = h.category.clone().unwrap_or_else(|| match h.scope {
        HolidayScope::National => "Feriados nacionais".into(),
        HolidayScope::Health => "Datas comemorativas da saúde".into(),
        other => other.as_str().into(),
    });
    let category_color = h.category_color.clone().unwrap_or_else(|| "#C0392B".into());

    json!({
        "date": format!("{:02}-{:02}", h.date.day(), h.date.month()),
        "title": h.title,
        "description": h.description,
        "legislation": h.legislation,
        "type": match h.kind {
            crate::holidays::HolidayKind::Feriado => "feriado",
            crate::holidays::HolidayKind::Facultativo => "facultativo",
            crate::holidays::HolidayKind::Commemorative => "commemorative",
        },
        "category": category,
        "categoryColor": category_color,
        "scope": h.scope.as_str(),
        "isoDate": h.date.to_string(),
    })
}
