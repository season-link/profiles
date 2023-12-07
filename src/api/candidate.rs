use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use axum_valid::Valid;
use sqlx::{Row};
use uuid::Uuid;

use crate::{
    api::{
        dtos::candidate::CreateCandidate,
        utils::{check_job_valid, create_keycloak_user},
    },
    SharedState,
};

use super::{
    dtos::{
        candidate::{Candidate, SimpleCandidate},
        list_candidate::{ListCandidate, SubscriptionLevel},
    },
    middlewares::auth_headers::AuthHeaders,
    utils::{check_query_effective, AppError},
};

/// Create the candidate inside the DB
#[debug_handler]
pub async fn create_candidate(
    State(state): State<Arc<SharedState>>,
    Valid(Json(mut dto)): Valid<Json<CreateCandidate>>,
) -> axum::response::Result<Json<Candidate>, AppError> {
    println!("{:?}", dto);

    check_job_valid(&dto.candidate.job_id).await?;
    dto.candidate.id = Some(create_keycloak_user(&dto).await?);

    sqlx::query("insert into candidate values( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15);")
    .bind(dto.candidate.id)
    .bind(&dto.candidate.first_name)
    .bind(&dto.candidate.last_name)
    .bind(dto.candidate.birth_date)
    .bind(&dto.candidate.nationality_country_id)
    .bind(&dto.candidate.description)
    .bind(&dto.candidate.email)
    .bind(&dto.candidate.phone_number)
    .bind(&dto.candidate.address)
    .bind(dto.candidate.gender)
    .bind(dto.candidate.is_available)
    .bind(dto.candidate.available_from)
    .bind(dto.candidate.available_to)
    .bind(&dto.candidate.place)
    .bind(dto.candidate.job_id)
    .execute(&state.pool).await?;

    Ok(Json(dto.candidate))
}

/// Retrieve the specified candidate
pub async fn get_candidate(
    State(state): State<Arc<SharedState>>,
    Path(user_id): Path<Uuid>,
    AuthHeaders {
        user_id: _,
        roles: _,
        request_id: _,
    }: AuthHeaders,
) -> axum::response::Result<Json<Candidate>, AppError> {
    get_candidate_db(state, user_id).await
}

/// Retrieve the logged-in candidate
pub async fn get_candidate_self(
    State(state): State<Arc<SharedState>>,
    AuthHeaders {
        user_id: user_uuid,
        roles: _,
        request_id: _,
    }: AuthHeaders,
) -> axum::response::Result<Json<Candidate>, AppError> {
    get_candidate_db(state, user_uuid).await
}

/// Get the candidate from the db
async fn get_candidate_db(
    state: Arc<SharedState>,
    user_id: Uuid,
) -> axum::response::Result<Json<Candidate>, AppError> {
    let candidate_result =
        sqlx::query_as::<_, Candidate>("select * from candidate c where c.id=$1 ;")
            .bind(user_id)
            .fetch_one(&state.pool)
            .await?;

    Ok(Json(candidate_result))
}

/// List candidates according to some filters
#[debug_handler]
pub async fn get_candidates(
    State(state): State<Arc<SharedState>>,
    Valid(Json(list_candidate)): Valid<Json<ListCandidate>>,
) -> axum::response::Result<Json<Vec<SimpleCandidate>>, AppError> {
    let limit: i16;
    match list_candidate.subscription_level {
        SubscriptionLevel::Free | SubscriptionLevel::Silver => {
            limit = 3;
        }
        SubscriptionLevel::Gold | SubscriptionLevel::Platinium => {
            limit = 10000;
        }
    }

    let result = sqlx::query(
        "select * from candidate c 
    where c.is_available = true 
    and c.job_id=$1 
    and $2 >= c.available_from  and $2 <= c.available_to 
    and $3 >= c.available_from  and $3 <= c.available_to 
    LIMIT $4;",
    )
    //.bind(&fields)
    .bind(list_candidate.job_id)
    .bind(list_candidate.start_date)
    .bind(list_candidate.end_date)
    .bind(limit)
    .map(|row| {
        
        SimpleCandidate {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            available_from: row.get("available_from"),
            available_to: row.get("available_to"),
            phone_number: if list_candidate.subscription_level == SubscriptionLevel::Free {
                None
            } else {
                row.get("phone_number")
            },
            email: if list_candidate.subscription_level == SubscriptionLevel::Free {
                None
            } else {
                row.get("email")
            },
        }
    })
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(result))
}

/// Update the candidate inside the DB
pub async fn update_candidate(
    State(state): State<Arc<SharedState>>,
    AuthHeaders {
        user_id: user_uuid,
        roles: _,
        request_id: _,
    }: AuthHeaders,
    Valid(Json(candidate)): Valid<Json<Candidate>>,
) -> axum::response::Result<Json<Candidate>, AppError> {
    check_job_valid(&candidate.job_id).await?;

    // The id inside the object is ignored, only the logged in ID matters
    let result = sqlx::query("UPDATE public.candidate
      SET first_name=$1, last_name=$2, birth_date=$3, nationality_country_id=$4, description=$5, email=$6, phone_number=$7, address=$8, gender=$9, is_available=$10, available_from=$11, available_to=$12, place=$13, job_id=$14
      WHERE id=$15 ;")
      .bind(&candidate.first_name)
      .bind(&candidate.last_name)
      .bind(candidate.birth_date)
      .bind(&candidate.nationality_country_id)
      .bind(&candidate.description)
      .bind(&candidate.email)
      .bind(&candidate.phone_number)
      .bind(&candidate.address)
      .bind(candidate.gender)
      .bind(candidate.is_available)
      .bind(candidate.available_from)
      .bind(candidate.available_to)
      .bind(&candidate.place)
      .bind(candidate.job_id)
      .bind(user_uuid)
      .execute(&state.pool).await?;

    check_query_effective(result)?;

    Ok(Json(candidate))
}

/// Delete the candidate inside the DB
pub async fn delete_candidate(
    State(state): State<Arc<SharedState>>,
    Path(user_id): Path<Uuid>,
) -> axum::response::Result<(), AppError> {
    let result = sqlx::query("DELETE FROM public.candidate WHERE id=$1;")
        .bind(user_id)
        .execute(&state.pool)
        .await?;

    check_query_effective(result)?;

    Ok(())
}
