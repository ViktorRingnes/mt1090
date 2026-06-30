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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn maps_each_variant_to_status() {
        let cases = [
            (FilmottakError::InvalidFilename, StatusCode::BAD_REQUEST),
            (FilmottakError::UnknownAvsenderkode("x".into()), StatusCode::BAD_REQUEST),
            (FilmottakError::Write(io::Error::other("disk")), StatusCode::INTERNAL_SERVER_ERROR),
        ];
        for (err, expected) in cases {
            assert_eq!(err.into_response().status(), expected);
        }
    }
}
