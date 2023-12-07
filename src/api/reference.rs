use anyhow::anyhow;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use axum_valid::Valid;
use uuid::Uuid;

use crate::SharedState;

use super::{
    dtos::reference::Reference,
    middlewares::auth_headers::AuthHeaders,
    utils::{check_query_effective, AppError},
};

/// Create the reference
pub async fn create_reference(
    State(state): State<Arc<SharedState>>,
    AuthHeaders {
        user_id: user_uuid,
        roles: _,
        request_id: _,
    }: AuthHeaders,
    Valid(Json(reference)): Valid<Json<Reference>>,
) -> axum::response::Result<Json<Reference>, AppError> {
    println!("{:?}", reference);

    sqlx::query("insert into reference values ($1, $2, $3, $4, $5, $6, $7);")
        .bind(reference.id)
        .bind(user_uuid)
        .bind(&reference.first_name)
        .bind(&reference.last_name)
        .bind(&reference.email)
        .bind(&reference.phone_number)
        .bind(&reference.company_name)
        .execute(&state.pool)
        .await?;

    // TODO verify candidate ID
    Ok(Json(reference))
}

/// Retrieve references of a given user inside the DB
pub async fn get_references(
    State(state): State<Arc<SharedState>>,
    Path(user_id): Path<Uuid>,
) -> axum::response::Result<Json<Vec<Reference>>, AppError> {
    let result = sqlx::query_as::<_, Reference>(
        "
    select id, first_name, last_name, email, phone_number, company_name from reference r
    where r.candidate_id=$1;",
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(result))
}

/// Retrieve the reference inside the DB
pub async fn get_reference(
    State(state): State<Arc<SharedState>>,
    Path(reference_id): Path<Uuid>,
) -> axum::response::Result<Json<Reference>, AppError> {
    let reference_query =
        sqlx::query_as::<_, Reference>("select id, first_name, last_name, email, phone_number, company_name from reference r where r.id=$1;")
            .bind(reference_id)
            .fetch_one(&state.pool)
            .await;

    match reference_query {
        Ok(reference) => Ok(Json(reference)),
        // FIXME it returns status code 500 lmao
        Err(_) => Err(AppError(anyhow!(
            "The reference does not exist: {}",
            reference_id
        ))),
    }
}

/// Update the reference
pub async fn update_reference(
    State(state): State<Arc<SharedState>>,
    AuthHeaders {
        user_id: user_uuid,
        roles: _,
        request_id: _,
    }: AuthHeaders,
    Path(reference_id): Path<Uuid>,
    Valid(Json(reference)): Valid<Json<Reference>>,
) -> axum::response::Result<Json<Reference>, AppError> {
    // Note, we do not update the candidate_id either
    let result = sqlx::query(
        "UPDATE public.reference
        SET first_name=$1, last_name=$2, email=$3, phone_number=$4, company_name=$5
        WHERE id=$6 and candidate_id=$7;",
    )
    .bind(&reference.first_name)
    .bind(&reference.last_name)
    .bind(&reference.email)
    .bind(&reference.phone_number)
    .bind(&reference.company_name)
    .bind(reference_id)
    .bind(user_uuid)
    .execute(&state.pool)
    .await?;

    check_query_effective(result)?;

    Ok(Json(reference))
}

/// Delete the reference
pub async fn delete_reference(
    State(state): State<Arc<SharedState>>,
    Path(reference_id): Path<Uuid>,
    AuthHeaders {
        user_id: user_uuid,
        roles: _,
        request_id: _,
    }: AuthHeaders,
) -> axum::response::Result<(), AppError> {
    let result = sqlx::query("DELETE FROM public.reference r WHERE r.id=$1 and r.candidate_id=$2;")
        .bind(reference_id)
        .bind(user_uuid)
        .execute(&state.pool)
        .await?;

    check_query_effective(result)?;

    Ok(())
}
