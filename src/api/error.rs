use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::domain::FilmottakError;

impl IntoResponse for FilmottakError {
    fn into_response(self) -> Response {
        let status = match &self {
            FilmottakError::InvalidFilename | FilmottakError::UnknownAvsenderkode(_) => {
                StatusCode::BAD_REQUEST
            }
            FilmottakError::Write(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}
