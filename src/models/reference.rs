use sqlx::types::Uuid;

pub struct Reference {
    id: Uuid,
    candidate_id: Uuid,
    first_name: String,
    last_name: String,
    mail: String,
    phone_country_id: String,
    phone_number: String,
    company_name: String,
}
