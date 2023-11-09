use std::sync::Arc;

use api::{
    candidate::{create_candidate, delete_candidate, get_candidate, update_candidate},
    utils::handler_404,
};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{pool, postgres::PgPoolOptions, Pool, Postgres};

mod api;
mod models;

#[derive(Debug)]
pub struct SharedState {
    pub pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //Access the DB
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://profiles:profiles@localhost/profiles")
        .await?;

    let shared_pool = Arc::new(SharedState { pool });

    // build our application with a single route
    let app = Router::new()
        .route("/", post(create_candidate))
        .route(
            "/:user_id",
            get(get_candidate)
                .put(update_candidate)
                .delete(delete_candidate),
        )
        .fallback(handler_404)
        .with_state(shared_pool);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
