use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Order not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    BadRequest(String),
    #[error("Validation failed")]
    Validation(#[from] ValidationErrors),
    #[error("Internal server error")]
    Internal,
}

#[derive(Serialize)]
struct ErrorResponse<T: Serialize> {
    message: String,
    details: Option<T>
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => {
                let body = Json(ErrorResponse::<()> {
                    message: "Order not found".into(),
                    details: None,
                });
                (StatusCode::NOT_FOUND, body).into_response()
            }
            ApiError::BadRequest(msg) => {
                let body = Json(ErrorResponse::<()> {
                    message: format!("Invalid input: {msg}"),
                    details: None,
                });
                (StatusCode::BAD_REQUEST, body).into_response()
            }
            ApiError::Validation(errs) => {
                let details = errs
                    .field_errors()
                    .iter()
                    .map(|(field, errors)| {
                        let messages: Vec<String> = errors
                            .iter()
                            .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                            .collect();
                        (field.to_string(), serde_json::Value::Array(
                            messages.into_iter().map(serde_json::Value::String).collect()
                        ))
                    })
                    .collect::<serde_json::Map<String, _>>();

                let body = Json(ErrorResponse {
                    message: "Validation failed".into(),
                    details: Some(details),
                });
                (StatusCode::BAD_REQUEST, body).into_response()
            }
            ApiError::Internal => {
                let body = Json(ErrorResponse::<()> {
                    message: "Internal server error".into(),
                    details: None,
                });
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use validator::ValidationErrors;

    #[test]
    fn test_api_error_display() {
        assert_eq!(ApiError::NotFound.to_string(), "Order not found");
        assert_eq!(ApiError::BadRequest("test".to_string()).to_string(), "Invalid input: test");
        assert_eq!(ApiError::Internal.to_string(), "Internal server error");
    }

    #[test]
    fn test_not_found_response() {
        let response = ApiError::NotFound.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_bad_request_response() {
        let response = ApiError::BadRequest("test error".to_string()).into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_internal_error_response() {
        let response = ApiError::Internal.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_validation_error_response() {
        let mut errors = ValidationErrors::new();
        errors.add("field", validator::ValidationError {
            code: std::borrow::Cow::Borrowed("length"),
            message: Some(std::borrow::Cow::Borrowed("too short")),
            params: std::collections::HashMap::new(),
        });
        
        let response = ApiError::Validation(errors).into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_validation_error_from_validation_errors() {
        let mut errors = ValidationErrors::new();
        errors.add("field", validator::ValidationError {
            code: std::borrow::Cow::Borrowed("length"),
            message: Some(std::borrow::Cow::Borrowed("too short")),
            params: std::collections::HashMap::new(),
        });

        let api_error: ApiError = errors.into();
        assert!(matches!(api_error, ApiError::Validation(_)));
    }
}