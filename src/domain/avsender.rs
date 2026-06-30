use crate::domain::FilmottakError;

pub enum AvsenderKoder {
    Eksamensdata,
    Studentstatus,
    KlargjorkundesakVedlegg,
    EHenvendelseVedlegg,
    StotterettVedlegg,
    AflKodeVedlegg,
}

pub fn valider_avsender_kode(kode: &str) -> Result<AvsenderKoder, FilmottakError> {
   match kode {
       "Eksamensdata" => Ok(AvsenderKoder::Eksamensdata),
       "Studentstatus" => Ok(AvsenderKoder::Studentstatus),
       "KlargjorkundesakVedlegg" => Ok(AvsenderKoder::KlargjorkundesakVedlegg),
       "EHenvendelseVedlegg" => Ok(AvsenderKoder::EHenvendelseVedlegg),
       "StotterettVedlegg" => Ok(AvsenderKoder::StotterettVedlegg),
       "AflKodeVedlegg" => Ok(AvsenderKoder::AflKodeVedlegg),
       _ => Err(FilmottakError::UnknownAvsenderkode(kode.to_string()))
   } 
}
