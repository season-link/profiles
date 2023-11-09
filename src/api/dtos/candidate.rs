use chrono::Local;
use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct Candidate {
    pub id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub first_name: String,
    #[validate(length(min = 1, max = 255))]
    pub last_name: String,
    pub birth_date: chrono::NaiveDateTime,
    pub nationality_country_id: String,
    pub description: String,

    #[validate(email)]
    pub email: String,
    pub phone_country_id: String,
    pub phone_number: String,
    pub adress: String,
    pub gender: i16,

    pub is_available: bool,
    pub available_from: chrono::NaiveDateTime,
    pub available_to: chrono::NaiveDateTime,
    pub place: String,
    pub job_id: Uuid,
}
