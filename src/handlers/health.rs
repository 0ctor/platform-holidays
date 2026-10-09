use crate::AppState;
use actix_web::{http::header, web, HttpResponse};
use serde_json::json;

fn standard_health_response(service: &str, ready: bool) -> HttpResponse {
    let mut response = if ready {
        HttpResponse::Ok()
    } else {
        HttpResponse::ServiceUnavailable()
    };

    response
        .insert_header((header::CACHE_CONTROL, "no-store"))
        .json(json!({
            "status": if ready { "ok" } else { "not_ready" },
            "service": service
        }))
}

pub async fn health_live(state: web::Data<AppState>) -> HttpResponse {
    standard_health_response(&state.service_name, true)
}

pub async fn health_ready(state: web::Data<AppState>) -> HttpResponse {
    // Motor de feriados é puro (sem DB). Ready = processo no ar.
    standard_health_response(&state.service_name, true)
}
