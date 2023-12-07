use std::{borrow::Cow, cmp::Ordering, collections::HashMap};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Serialize, Deserialize, sqlx::FromRow)]
#[validate(schema(function = "validate_category", skip_on_field_errors = false))]
pub struct Candidate {
    pub id: Option<Uuid>,
    #[validate(length(min = 1, max = 255))]
    pub first_name: String,
    #[validate(length(min = 1, max = 255))]
    pub last_name: String,
    pub birth_date: chrono::NaiveDateTime,
    pub nationality_country_id: String,
    #[validate(length(min = 0, max = 255))]
    pub description: String,

    #[validate(email)]
    pub email: String,
    #[validate(phone)]
    pub phone_number: String,
    pub address: String,
    pub gender: i16,

    pub is_available: bool,
    pub available_from: chrono::NaiveDateTime,
    pub available_to: chrono::NaiveDateTime,
    pub place: String,
    pub job_id: Uuid,
}

// Create user Dto
#[derive(Debug, Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct CreateCandidate {
    #[validate]
    #[serde(flatten)]
    pub candidate: Candidate,

    #[validate(length(min = 4))]
    pub password: String,
}

/// A simpler version of the candidate, used to list stuff
#[derive(Debug, Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct SimpleCandidate {
    pub id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub first_name: String,
    #[validate(length(min = 1, max = 255))]
    pub last_name: String,

    #[validate(email)]
    pub email: Option<String>,
    #[validate(phone)]
    pub phone_number: Option<String>,

    pub available_from: chrono::NaiveDateTime,
    pub available_to: chrono::NaiveDateTime,
}

/// Global struct validation
fn validate_category(candidate: &Candidate) -> Result<(), ValidationError> {
    if candidate.available_from.cmp(&candidate.available_to) != Ordering::Less {
        return Err(ValidationError {
            code: Cow::from("Candidate"),
            message: Some(Cow::from("available from is not smaller than available to")),
            params: HashMap::new(),
        });
    }

    Ok(())
}
