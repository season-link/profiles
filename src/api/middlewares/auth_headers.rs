use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Role {
    Candidate,
    Admin,
    Unknown,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        if value == "client_candidate" {
            Role::Candidate
        } else if value == "client_admin" {
            return Role::Admin;
        } else {
            return Role::Unknown;
        }
    }
}

/// Auth headers from the API Gateway
#[derive(Debug)]
pub struct AuthHeaders {
    pub user_id: Uuid,
    pub roles: Vec<Role>,
    pub request_id: Uuid,
}

/// Extract the auth headers from the request
#[async_trait]
impl<S> FromRequestParts<S> for AuthHeaders
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_uuid: Uuid;
        if let Some(user_id) = parts.headers.get("X-User-Id") {
            match Uuid::parse_str(user_id.to_str().unwrap_or_default()) {
                Ok(id) => user_uuid = id,
                Err(_) => return Err((StatusCode::BAD_REQUEST, "`X-User-Id` header is malformed")),
            }
        } else {
            return Err((StatusCode::BAD_REQUEST, "`X-User-Id` header is missing"));
        }

        let parsed_roles: Vec<Role>;
        if let Some(roles) = parts.headers.get("X-User-Roles") {
            parsed_roles = roles
                .to_str()
                .unwrap_or_default()
                .split(',')
                .collect::<Vec<_>>()
                .iter()
                .map(|role| Role::from(role.to_owned()))
                .collect::<Vec<_>>();
        } else {
            return Err((StatusCode::BAD_REQUEST, "`X-User-Roles` header is missing"));
        }

        let request_uuid: Uuid;
        if let Some(request_id) = parts.headers.get("X-Request-Id") {
            match Uuid::parse_str(request_id.to_str().unwrap_or_default()) {
                Ok(id) => request_uuid = id,
                Err(_) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        "`X-Request-Id` header is malformed",
                    ))
                }
            }
        } else {
            return Err((StatusCode::BAD_REQUEST, "`X-Request-Id` header is missing"));
        }

        Ok(AuthHeaders {
            user_id: user_uuid,
            request_id: request_uuid,
            roles: parsed_roles,
        })
    }
}
