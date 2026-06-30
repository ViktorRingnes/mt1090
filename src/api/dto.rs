use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LagreFilB2BParameter {
    #[serde(deserialize_with = "base64_bytes")]
    pub fil: Vec<u8>,
    pub filnavn: String,
    pub avsenderkode: String,
}

fn base64_bytes<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    let s = String::deserialize(deserializer)?;
    STANDARD.decode(s).map_err(serde::de::Error::custom)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LagreFilB2BRetur {
    pub filreferanse: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilinformasjonType {
    pub filreferanse: String,
    pub avsenderkode: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HentUNCForFilerParameter {
    pub filinformasjon_liste: Vec<FilinformasjonType>,
}

#[derive(Debug, Serialize)]
pub struct HentUNCForFilerRetur(pub Vec<String>);

#[derive(Debug, Serialize)]
pub struct PingRetur {
    pub message: String,
}
