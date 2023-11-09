use sqlx::types::{time::Date, Uuid};

pub struct Experience {
    id: Uuid,
    candidate_id: Uuid,
    company_name: String,
    job_id: Uuid,
    start_time: Date,
    end_time: Date,
    description: String,
}
