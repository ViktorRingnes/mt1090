use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::api::Response;

pub async fn lagre_fil_b2b_handler() -> impl IntoResponse {
    StatusCode::OK
}


pub async fn hent_unc_for_filer_handler() -> impl IntoResponse {
    StatusCode::OK
}


pub async fn ping_handler() -> impl IntoResponse {
    Json(Response {
        message: "pong".to_string()
    })
}
