use crate::domain::FilmottakError;

pub fn store_file(file_bytes: Vec<u8>) -> Result<String, FilmottakError> {
    Ok("hello world".to_string())
}
