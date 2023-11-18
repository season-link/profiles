use sqlx::types::{time, Uuid};

pub struct Candidate {
    id: Uuid,
    first_name: String,
    last_name: String,
    birth_date: time::Date,
    nationality_country_id: String,
    description: String,

    email: String,
    phone_country_id: String,
    phone_number: String,
    adress: String,
    gender: i8,

    is_available: bool,
    available_from: time::Date,
    available_to: time::Date,
    place: String,
    job_id: Uuid,
}
