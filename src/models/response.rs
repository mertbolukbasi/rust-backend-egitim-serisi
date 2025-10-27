use std::fmt::{Debug, Display, Formatter};
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ApiResponse<T: Serialize> {
	pub success: bool,
	pub status_code: u16,
	pub body: T,
	pub timestamp: DateTime<Utc>
}

impl<T: Serialize> ApiResponse<T> {
	pub fn success(status_code: u16, body: T) -> Self {
		ApiResponse {
			success: true,
			status_code,
			body,
			timestamp: Utc::now()
		}
	}
	pub fn error(status_code: u16, body: T) -> Self {
		ApiResponse {
			success: false,
			status_code,
			body,
			timestamp: Utc::now()
		}
	}
}

impl<T: Serialize> Responder for ApiResponse<T> {
	type Body = BoxBody;

	fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
		let body = serde_json::to_string(&self).unwrap();
		HttpResponse::build(
			StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
		)
			.content_type("application/json")
			.body(body)
	}
}

impl<T: Serialize> Display for ApiResponse<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", serde_json::to_string(&self).unwrap())
	}
}

impl ResponseError for ApiResponse<String> {
	fn status_code(&self) -> StatusCode {
		StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
	}

	fn error_response(&self) -> HttpResponse {
		let body = serde_json::to_string(&self).unwrap();
		HttpResponse::build(StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
			.content_type("application/json")
			.body(body)
	}
}