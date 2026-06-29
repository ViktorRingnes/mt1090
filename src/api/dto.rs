use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LagreFilB2BParameter {
    pub fil: Vec<u8>,
    pub filnavn: String,
    pub avsenderkode: String,
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
