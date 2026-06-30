use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    api::FilinformasjonType,
    config,
    domain::{self, FilmottakError},
};

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

fn folder_for(avsenderkode: &str) -> Result<PathBuf, FilmottakError> {
    let subfolder = domain::avsender::valider_avsender_kode(avsenderkode)?;
    Ok(Path::new(&config::get().rotmappe).join(subfolder))
}


pub fn resolve_uncs(items: Vec<FilinformasjonType>) -> Result<Vec<String>, FilmottakError> {
    items.iter()
        .map(|f| resolve_unc(&f.avsenderkode, &f.filreferanse))
        .collect()
}

fn resolve_unc(avsenderkode: &str, filreferanse: &str) -> Result<String, FilmottakError> {
    let folder = folder_for(avsenderkode)?;
    let Ok(entries) = fs::read_dir(&folder) else { return Ok(String::new()) };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.to_string_lossy().ends_with(filreferanse) {
            return Ok(path.to_string_lossy().into_owned());
        }
    }
    Ok(String::new())
}
