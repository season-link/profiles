use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct Reference {
    pub id: Uuid,
    //pub candidate_id: Uuid, // Should be invisible to the user
    #[validate(length(min = 1, max = 255))]
    pub first_name: String,
    #[validate(length(min = 1, max = 255))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(phone)]
    pub phone_number: String,
    #[validate(length(min = 1, max = 255))]
    pub company_name: String,
}
