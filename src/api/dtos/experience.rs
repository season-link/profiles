use std::{borrow::Cow, cmp::Ordering, collections::HashMap};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Serialize, Deserialize, sqlx::FromRow)]
#[validate(schema(function = "validate_category", skip_on_field_errors = false))]
pub struct Experience {
    pub id: Uuid,
    pub candidate_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub company_name: String,
    pub job_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    #[validate(length(min = 1, max = 255))]
    pub description: String,
}

/// Global struct validation
fn validate_category(experience: &Experience) -> Result<(), ValidationError> {
    if experience.start_time.cmp(&experience.end_time) != Ordering::Less {
        return Err(ValidationError {
            code: Cow::from("Experience"),
            message: Some(Cow::from("start_date is not smaller than end_date")),
            params: HashMap::new(),
        });
    }

    Ok(())
}
