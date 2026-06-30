use crate::{config, domain::FilmottakError};

pub fn valider_avsender_kode(kode: &str) -> Result<String, FilmottakError> {
    config::get()
        .avsenderkoder
        .get(kode)
        .cloned()
        .ok_or_else(|| FilmottakError::UnknownAvsenderkode(kode.to_string()))
}
