use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{api::{HentUNCForFilerParameter, LagreFilB2BParameter, LagreFilB2BRetur, PingRetur}, domain};

pub async fn lagre_fil_b2b_handler(Json(data): Json<LagreFilB2BParameter>) -> impl IntoResponse {
    let filreferanse = domain::filmottak::store_file(data.fil).unwrap();

    Json(LagreFilB2BRetur {
        filreferanse
    })
}

pub async fn hent_unc_for_filer_handler(_: Json<HentUNCForFilerParameter>) -> impl IntoResponse {
    StatusCode::OK
}

pub async fn ping_handler() -> impl IntoResponse {
    Json(PingRetur {
        message: "pong".to_string(),
    })
}
