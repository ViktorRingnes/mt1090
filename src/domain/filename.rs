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
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn ok_iff_no_separator(navn in ".*") {
            match valider_filnavn(&navn) {
                Ok(out) => prop_assert!(!navn.contains(SEPARATOR) && out == navn),
                Err(_) => prop_assert!(navn.contains(SEPARATOR)),
            }
        }
    }
}
