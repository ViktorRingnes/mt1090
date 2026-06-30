use crate::domain::FilmottakError;

const SEPARATOR: &str = "##";

pub fn valider_filnavn(navn: &str) -> Result<String, FilmottakError> {
    if navn.contains(SEPARATOR) {
        return Err(FilmottakError::InvalidFilename);
    }
    Ok(navn.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_filnavn_with_separator() {
        assert!(valider_filnavn("rapport.txt").is_ok());
        assert!(valider_filnavn("rap##ort.txt").is_err());
    }
}
