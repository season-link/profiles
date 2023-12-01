use dotenvy::dotenv;
use std::{env, sync::Arc};

use api::{
    candidate::{
        create_candidate, delete_candidate, get_candidate, get_candidate_self, get_candidates,
        update_candidate,
    },
    experience::{create_experience, get_experiences, update_experience},
    middlewares::is_admin::is_admin,
    reference::{create_reference, delete_reference, get_reference, get_references},
    utils::handler_404,
};
use axum::{
    handler::Handler,
    middleware,
    routing::{get, post, put},
    Router,
};
use sqlx::{pool, postgres::PgPoolOptions, Pool, Postgres};

mod api;

#[derive(Debug)]
pub struct SharedState {
    pub pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load environment variables from .env file
    match dotenv() {
        Ok(_) => println!("LOG: Env file loaded"),
        Err(_) => println!("WARN: no env file found, is this expected ?"),
    }

    // Check env vars
    let postgres_url = env::var("db_url").expect("No database url !");
    let server_port = env::var("server_port").unwrap_or(String::from("3000"));

    //Access the DB
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await?;

    let shared_pool = Arc::new(SharedState { pool });

    // build our applications
    let mut router = Router::new();

    // Register the users
    router = router
        .route("/user", post(create_candidate))
        .route("/users", get(get_candidates))
        .route("/user/me", put(update_candidate).get(get_candidate_self))
        .route(
            "/user/:user_id",
            get(get_candidate).delete(delete_candidate.layer(middleware::from_fn(is_admin))),
        );

    // Register the references
    router = router
        .route("/user/:user_id/references", get(get_references))
        .route("/references", post(create_reference))
        .route(
            "/reference/:reference_id",
            get(get_reference)
                .delete(delete_reference)
                .put(update_candidate),
        );

    // Register the experiences
    router = router
        .route("/user/:user_id/experiences", get(get_experiences))
        .route("/experience", post(create_experience))
        .route(
            "/experience/:experience_id",
            get(get_reference)
                .delete(delete_reference)
                .put(update_experience),
        );

    // Register fallback
    router = router.fallback(handler_404);

    let app = router.with_state(shared_pool);

    // run it with hyper on localhost:3000
    axum::Server::bind(
        &(String::from("0.0.0.0:") + &server_port)
            .parse()
            .expect("Malformed server url !"),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())
}
