use std::{fs, path::Path};

use crate::{config, domain::{self, FilmottakError}};

pub fn store_file(
    file_bytes: Vec<u8>,
    filnavn: &str,
    avsenderkode: &str,
) -> Result<String, FilmottakError> {
    domain::filename::valider_filnavn(filnavn)?;
    let subfolder = domain::avsender::valider_avsender_kode(avsenderkode)?;
    let reference = domain::reference::generate_reference();

    let cfg = config::get();
    let folder = Path::new(&cfg.rotmappe).join(subfolder);
    let file_path = folder.join(format!("{filnavn}{}{reference}", cfg.separator));

    write_to_disk(&folder, &file_path, &file_bytes)?;
    Ok(reference)
}

fn write_to_disk(folder: &Path, file_path: &Path, bytes: &[u8]) -> Result<(), FilmottakError> {
    fs::create_dir_all(folder)?;
    fs::write(file_path, bytes)?;
    Ok(())
}
