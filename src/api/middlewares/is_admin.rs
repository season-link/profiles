use std::ops::Deref;

use axum::{http::{Request}, middleware::Next, response::Response};
use anyhow::anyhow;
use crate::api::utils::AppError;

use super::auth_headers::{AuthHeaders, Role};

/// Check whether the user is an admin
pub async fn is_admin<T>(
    AuthHeaders {
        user_id: _,
        roles,
        request_id: _,
    }: AuthHeaders,
    request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    // Can probably be generalized for roles
    if !roles.iter().any(|role| role.to_owned() == Role::Admin) {
        return Err(AppError(anyhow!(
            "The user does not have one of the following roles: {:?}",
            Role::Admin
        )));
    }

    Ok(next.run(request).await)
}