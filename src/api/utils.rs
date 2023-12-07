use std::{env};

use anyhow::anyhow;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};


use serde_json::json;
use sqlx::{postgres::PgQueryResult};
use uuid::Uuid;



use super::dtos::candidate::{CreateCandidate};

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
        return Err(AppError(anyhow!(
            "Too many ressources affected: {} rows affected",
            query_result.rows_affected()
        )));
    }

    Ok(())
}

/// Throws an app error on invalid job id
pub async fn check_job_valid(job_id: &Uuid) -> Result<(), AppError> {
    let is_valid = is_job_uuid_valid(job_id).await?;
    if !is_valid {
        return Err(AppError(anyhow!("Invalid job id : {} ", job_id)));
    }
    Ok(())
}

/// Check whether the job uuid is a valid one
pub async fn is_job_uuid_valid(job_id: &Uuid) -> Result<bool, AppError> {
    let root_url = env::var("job_service_url").expect("No job service URL !");
    let response = reqwest::get(root_url + "/jobs/" + &job_id.to_string()).await?;
    println!("Job check: {}", &response.status());
    Ok(StatusCode::is_success(&response.status()))
}

/// Get the token to execute operations on the IDP
pub async fn refresh_token() -> Result<String, AppError> {
    let root_url = env::var("keycloak_url").expect("No keycloak URL !");
    let acount_username =
        env::var("keycloak_service_account_username").expect("No keycloak username !");
    let account_password =
        env::var("keycloak_service_account_password").expect("No keycloak password !");
    let client_id = env::var("keycloak_client_id").expect("No keycloak client id !");
    //TODO let' s not do a connection each time, transfer the token to the shared state
    let params = [
        ("grant_type", "password"),
        ("username", &acount_username),
        ("password", &account_password),
        ("client_id", &client_id),
    ];
    let client = reqwest::Client::new();
    let response = client
        .post(root_url + "/realms/season-link/protocol/openid-connect/token")
        .form(&params)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let token = response["access_token"].as_str().unwrap();

    Ok(token.to_owned())
}

/// Create a user inside the IDP
pub async fn create_keycloak_user(dto: &CreateCandidate) -> Result<Uuid, AppError> {
    let token = refresh_token().await?;
    let client = reqwest::Client::new();
    let root_url = env::var("keycloak_url").expect("No keycloak URL !");

    // First create the user
    let create_body = serde_json::json!({
        "enabled": true,
        "username": dto.candidate.first_name,
        "firstName": dto.candidate.first_name,
        "lastName": dto.candidate.last_name,
        "emailVerified": true
    });

    let create_response = client
        .post(root_url.clone() + "/admin/realms/season-link/users/")
        .json(&create_body)
        .header("Authorization", "Bearer ".to_owned() + &token)
        .send()
        .await?;

    if !create_response.status().is_success() {
        return Err(AppError(anyhow!(
            "Failed to create a user on keycloak: {}",
            create_response.status()
        )));
    }

    //FIXME yes there are a lot of unwraps
    let uuid_header = create_response
        .headers()
        .get("Location")
        .unwrap()
        .to_str()?;
    let uuid = Uuid::parse_str(uuid_header.rsplit_once('/').unwrap().1)?;

    println!("{}", &create_response.text().await?);

    let password_body = json!({
        "type": "password",
        "temporary": false,
        "value": dto.password
    });

    // Then set up the password
    let password_response = client
        .put(
            root_url.clone()
                + "/admin/realms/season-link/users/"
                + &uuid.to_string()
                + "/reset-password",
        )
        .json(&password_body)
        .header("Authorization", "Bearer ".to_owned() + &token)
        .send()
        .await?;

    if !password_response.status().is_success() {
        return Err(AppError(anyhow!(
            "Failed to set the user password : {}",
            password_response.status()
        )));
    }

    Ok(uuid)
}
