use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use axum_valid::Valid;
use uuid::Uuid;

use crate::SharedState;

use super::{
    dtos::experience::Experience,
    middlewares::auth_headers::AuthHeaders,
    utils::{check_query_effective, AppError},
};

pub async fn create_experience(
    State(state): State<Arc<SharedState>>,
    AuthHeaders {
        user_id: user_uuid,
        roles: _,
        request_id: _,
    }: AuthHeaders,
    Valid(Json(experience)): Valid<Json<Experience>>,
) -> axum::response::Result<Json<Experience>, AppError> {
    sqlx::query("insert into experience values ($1, $2, $3, $4, $5, $6, $7);")
        .bind(&experience.id)
        .bind(user_uuid)
        .bind(&experience.company_name)
        .bind(&experience.job_id)
        .bind(&experience.start_time)
        .bind(&experience.end_time)
        .bind(&experience.description)
        .execute(&state.pool)
        .await?;

    Ok(Json(experience))
}

pub async fn get_experiences(
    State(state): State<Arc<SharedState>>,
    AuthHeaders {
        user_id: user_uuid,
        roles: _,
        request_id: _,
    }: AuthHeaders,
) -> axum::response::Result<Json<Vec<Experience>>, AppError> {
    let result = sqlx::query_as::<_, Experience>(
        "select id, company_name, job_id, start_time, end_time, description
    FROM experience e
    WHERE e.candidate_id=$2;",
    )
    .bind(user_uuid)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(result))
}

pub async fn get_experience(
    State(state): State<Arc<SharedState>>,
    AuthHeaders {
        user_id: user_uuid,
        roles: _,
        request_id: _,
    }: AuthHeaders,
    Path(reference_id): Path<Uuid>,
) -> axum::response::Result<Json<Experience>, AppError> {
    let result = sqlx::query_as::<_, Experience>(
        "select id, company_name, job_id, start_time, end_time, description
    FROM experience e
    WHERE e.id=$1 and e.candidate_id=$2;",
    )
    .bind(reference_id)
    .bind(user_uuid)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(result))
}

pub async fn update_experience(
    State(state): State<Arc<SharedState>>,
    AuthHeaders {
        user_id: user_uuid,
        roles: _,
        request_id: _,
    }: AuthHeaders,
    Path(reference_id): Path<Uuid>,
    Valid(Json(experience)): Valid<Json<Experience>>,
) -> axum::response::Result<Json<Experience>, AppError> {
    let result = sqlx::query(
        "UPDATE reference
        SET company_name=$1, job_id=$2, start_time=$3, end_time=$4, description=$5
        WHERE id=$6 and candidate=$7;",
    )
    .bind(reference_id)
    .bind(user_uuid)
    .execute(&state.pool)
    .await?;

    check_query_effective(result)?;

    Ok(Json(experience))
}

pub async fn delete_experience(
    State(state): State<Arc<SharedState>>,
    AuthHeaders {
        user_id: user_uuid,
        roles: _,
        request_id: _,
    }: AuthHeaders,
    Path(reference_id): Path<Uuid>,
) -> axum::response::Result<(), AppError> {
    let result = sqlx::query(
        "
    DELETE FROM public.experience e 
    WHERE e.id=$1 and e.candidate_id=$2;",
    )
    .bind(reference_id)
    .bind(user_uuid)
    .execute(&state.pool)
    .await?;

    check_query_effective(result)?;

    Ok(())
}
