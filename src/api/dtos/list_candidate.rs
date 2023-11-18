use std::{borrow::Cow, cmp::Ordering, collections::HashMap};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

///The subscription level of the employer, trust the calling service I guess
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionLevel {
    Free,
    Silver,
    Gold,
    Platinium,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_category", skip_on_field_errors = false))]
/// List candidates according to the subscription
pub struct ListCandidate {
    pub job_id: Uuid,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub subscription_level: SubscriptionLevel,
}

fn validate_category(list_candidate: &ListCandidate) -> Result<(), ValidationError> {
    if list_candidate.start_date.cmp(&list_candidate.end_date) != Ordering::Less {
        return Err(ValidationError {
            code: Cow::from("List Candidate"),
            message: Some(Cow::from("start_date is not smaller than end_date")),
            params: HashMap::new(),
        });
    }

    Ok(())
}
