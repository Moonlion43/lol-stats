use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub enum AppError {
	Database(sqlx::Error),
	Template(askama::Error),
	BadRequest(&'static str),
}

impl From<sqlx::Error> for AppError {
	fn from(error: sqlx::Error) -> Self {
		Self::Database(error)
	}
}

impl From<askama::Error> for AppError {
	fn from(error: askama::Error) -> Self {
		Self::Template(error)
	}
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		let (status, message) = match self {
			Self::Database(error) => {
				eprintln!("database error: {error}");
				(StatusCode::INTERNAL_SERVER_ERROR, "Database error")
			}
			Self::Template(error) => {
				eprintln!("template error: {error}");
				(StatusCode::INTERNAL_SERVER_ERROR, "Template error")
			}
			Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
		};

		(status, message).into_response()
	}
}
