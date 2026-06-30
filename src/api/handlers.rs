use axum::{Json, response::IntoResponse};

use crate::{
    api::{HentUNCForFilerParameter, HentUNCForFilerRetur, LagreFilB2BParameter, LagreFilB2BRetur, PingRetur},
    domain::{self, FilmottakError},
};

pub async fn lagre_fil_b2b_handler(
    Json(data): Json<LagreFilB2BParameter>,
) -> Result<Json<LagreFilB2BRetur>, FilmottakError> {
    let filreferanse =
        domain::filmottak::store_file(data.fil, &data.filnavn, &data.avsenderkode)?;

    Ok(Json(LagreFilB2BRetur { filreferanse }))
}

pub async fn hent_unc_for_filer_handler(
    Json(data): Json<HentUNCForFilerParameter>,
) -> Result<Json<HentUNCForFilerRetur>, FilmottakError> {
    let uncs = domain::filmottak::resolve_uncs(data.filinformasjon_liste)?;
    Ok(Json(HentUNCForFilerRetur(uncs)))
}

pub async fn ping_handler() -> impl IntoResponse {
    Json(PingRetur {
        message: "pong".to_string(),
    })
}
