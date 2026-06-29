use std::fmt;

#[derive(Debug)]
pub enum FilmottakError {
    InvalidFilename,
    UnknownAvsenderkode(String),
    Write(std::io::Error),
}

impl fmt::Display for FilmottakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilmottakError::InvalidFilename => {
                write!(f, "Ugyldig filnavn. Inneholder ugyldige tegn")
            }
            FilmottakError::UnknownAvsenderkode(kode) => {
                write!(f, "Ukjent Avsenderkode: {kode}. Filen lagres ikke til disk")
            }
            FilmottakError::Write(e) => write!(f, "Lagring av fil feilet i MT1090: {e}"),
        }
    }
}

impl std::error::Error for FilmottakError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FilmottakError::Write(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for FilmottakError {
    fn from(e: std::io::Error) -> Self {
        FilmottakError::Write(e)
    }
}
