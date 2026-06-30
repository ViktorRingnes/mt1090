use crate::domain::FilmottakError::{self, InvalidFilename};

pub fn store_file(file_bytes: Vec<u8>) -> Result<String, FilmottakError> {
    if file_bytes.iter().len() > 5 {
        return Err(InvalidFilename)
    }

    Ok("hello world".to_string())
}
