use crate::{config, domain::FilmottakError};

pub fn valider_avsender_kode(kode: &str) -> Result<String, FilmottakError> {
    config::get()
        .avsenderkoder
        .get(kode)
        .cloned()
        .ok_or_else(|| FilmottakError::UnknownAvsenderkode(kode.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_known_code_and_rejects_unknown() {
        assert_eq!(valider_avsender_kode("Eksamensdata").unwrap(), "Eksamensdata");
        assert!(matches!(
            valider_avsender_kode("IkkeEnKode"),
            Err(FilmottakError::UnknownAvsenderkode(_))
        ));
    }
}
