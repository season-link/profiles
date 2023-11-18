use std::{error::Error, ops::Deref};

use anyhow::anyhow;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;
use sqlx::{database::HasValueRef, Database, Decode, postgres::PgQueryResult};

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(pub anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

/// 404 handler
pub async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}


/// Check if a request result affected only one row
pub fn check_query_effective(query_result: PgQueryResult) -> Result<(), AppError> {
    if query_result.rows_affected() == 0 {
        return Err(AppError(anyhow!("The resource does not exist")));
    }
    if query_result.rows_affected() > 1 {
        return Err(AppError(anyhow!("Too many ressources affected: {} rows affected", query_result.rows_affected())));
    }

    Ok(())
}
