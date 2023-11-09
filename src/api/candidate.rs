use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    Json,
};
use axum_valid::Valid;
use uuid::Uuid;

use crate::SharedState;

use super::{dtos::candidate::Candidate, extractors::auth_headers::AuthHeaders, utils::AppError};

/// Create the candidate inside the DB
pub async fn create_candidate(
    State(state): State<Arc<SharedState>>,
    Valid(Json(candidate)): Valid<Json<Candidate>>,
) -> axum::response::Result<Json<Candidate>, AppError> {
    println!("{:?}", candidate);

    sqlx::query("insert into candidate values( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16);")
    .bind(&candidate.id)
    .bind(&candidate.first_name)
    .bind(&candidate.last_name)
    .bind(&candidate.birth_date)
    .bind(&candidate.nationality_country_id)
    .bind(&candidate.description)
    .bind(&candidate.email)
    .bind(&candidate.phone_country_id)
    .bind(&candidate.phone_number)
    .bind(&candidate.adress)
    .bind(&candidate.gender)
    .bind(&candidate.is_available)
    .bind(&candidate.available_from)
    .bind(&candidate.available_to)
    .bind(&candidate.place)
    .bind(&candidate.job_id)
    .execute(&state.pool).await?;

    return Ok(Json(candidate));
}

/// Retrieve the candidate inside the DB
pub async fn get_candidate(
    State(state): State<Arc<SharedState>>,
    Path(user_id): Path<Uuid>,
    AuthHeaders {
        user_id: user_uuid,
        roles,
        request_id,
    }: AuthHeaders,
) -> axum::response::Result<Json<Candidate>, AppError> {
    println!("{:?}, {:?}, {:?}", user_uuid, roles, request_id);

    let candidate_query =
        sqlx::query_as::<_, Candidate>("select * from candidate c where c.id=$1 ;")
            .bind(user_id)
            .fetch_one(&state.pool)
            .await;

    match candidate_query {
        Ok(candidate) => Ok(Json(candidate)),
        // FIXME it returns status code 500 lmao
        Err(_) => Err(AppError(anyhow!(
            "The candidate does not exist: {}",
            user_id
        ))),
    }
}

/// Update the candidate inside the DB
pub async fn update_candidate(
    State(state): State<Arc<SharedState>>,
    Valid(Json(candidate)): Valid<Json<Candidate>>,
) -> axum::response::Result<Json<Candidate>, AppError> {
    sqlx::query("UPDATE public.candidate
      SET first_name=$1, last_name=$2, birth_date=$3, nationality_country_id=$4, description=$5, email=$6, phone_country_id=$7, phone_number=$8, adress=$9, gender=$10, is_available=$11, available_from=$12, available_to=$13, place=$14, job_id=$15
      WHERE id=$16 ;")
      .bind(&candidate.first_name)
      .bind(&candidate.last_name)
      .bind(&candidate.birth_date)
      .bind(&candidate.nationality_country_id)
      .bind(&candidate.description)
      .bind(&candidate.email)
      .bind(&candidate.phone_country_id)
      .bind(&candidate.phone_number)
      .bind(&candidate.adress)
      .bind(&candidate.gender)
      .bind(&candidate.is_available)
      .bind(&candidate.available_from)
      .bind(&candidate.available_to)
      .bind(&candidate.place)
      .bind(&candidate.job_id)
      .bind(&candidate.id)
      .execute(&state.pool).await?;

    return Ok(Json(candidate));
}

/// Delete the candidate inside the DB
pub async fn delete_candidate(
    State(state): State<Arc<SharedState>>,
    Path(user_id): Path<Uuid>,
) -> axum::response::Result<(), AppError> {
    sqlx::query("DELETE FROM public.candidate WHERE id=$1;")
        .bind(user_id)
        .execute(&state.pool)
        .await?;
    Ok(())
}
