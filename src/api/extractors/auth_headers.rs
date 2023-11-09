use std::ops::Deref;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderValue, StatusCode},
};
use uuid::{uuid, Uuid};

#[derive(Debug)]
pub enum Role {
    Candidate,
    Admin,
    Unknown,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        if value == "client_candidate" {
            return Role::Candidate;
        } else if value == "client_admin" {
            return Role::Admin;
        } else {
            return Role::Unknown;
        }
    }
}

#[derive(Debug)]
pub struct AuthHeaders {
    pub user_id: Uuid,
    pub roles: Vec<Role>,
    pub request_id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthHeaders
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user_uuid: Uuid;
        //FIXME remove unwraps
        if let Some(user_id) = parts.headers.get("X-User-Id") {
            user_uuid = Uuid::parse_str(user_id.to_str().unwrap_or_default()).unwrap();
        } else {
            return Err((StatusCode::BAD_REQUEST, "`X-User-Id` header is missing"));
        }

        let parsed_roles: Vec<Role>;
        if let Some(roles) = parts.headers.get("X-User-Roles") {
            parsed_roles = roles
                .to_str()
                .unwrap()
                .split(',')
                .collect::<Vec<_>>()
                .iter()
                .map(|role| Role::from(role.deref()))
                .collect::<Vec<_>>();
        } else {
            return Err((StatusCode::BAD_REQUEST, "`X-User-Roles` header is missing"));
        }

        let request_uuid: Uuid;
        if let Some(request_id) = parts.headers.get("X-Request-Id") {
            request_uuid = Uuid::parse_str(request_id.to_str().unwrap_or_default()).unwrap();
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
