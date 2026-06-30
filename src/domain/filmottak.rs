use crate::domain::{self, FilmottakError};

pub fn store_file(
    _file_bytes: Vec<u8>,
    filnavn: &str,
    avsenderkode: &str,
) -> Result<String, FilmottakError> {
    domain::filename::valider_filnavn(filnavn)?;
    domain::avsender::valider_avsender_kode(avsenderkode)?;

    Ok(domain::reference::generate_reference())
}
