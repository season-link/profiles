use aws_sdk_s3::{
    config::{Credentials, Region},
    Client,
};
use dotenvy::dotenv;
use std::{env, sync::Arc};

use api::{
    candidate::{
        create_candidate, delete_candidate, get_candidate, get_candidate_self, get_candidates,
        update_candidate,
    },
    cv::post_cv,
    experience::{
        create_experience, delete_experience, get_experience, get_experiences, update_experience,
    },
    middlewares::is_admin::is_admin,
    reference::{
        create_reference, delete_reference, get_reference, get_references, update_reference,
    },
    utils::handler_404,
};
use axum::{
    handler::Handler,
    middleware,
    routing::{get, post, put},
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::api::cv::{get_cv, get_cv_self};

mod api;

#[derive(Debug)]
pub struct SharedState {
    pub pool: Pool<Postgres>,
    pub s3_client: Client,
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

    let shared_pool = Arc::new(SharedState {
        pool,
        s3_client: client_to_s3().await,
    });

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

    // Register the user files operations
    router = router
        .route("/user/me/cv", post(post_cv).get(get_cv_self))
        .route("/user/:user_id/cv", get(get_cv));

    // Register the references
    router = router
        .route("/user/:user_id/references", get(get_references))
        .route("/references", post(create_reference))
        .route(
            "/reference/:reference_id",
            get(get_reference)
                .delete(delete_reference)
                .put(update_reference),
        );

    // Register the experiences
    router = router
        .route("/user/:user_id/experiences", get(get_experiences))
        .route("/experience", post(create_experience))
        .route(
            "/experience/:experience_id",
            get(get_experience)
                .delete(delete_experience)
                .put(update_experience),
        );

    // Register fallback
    router = router.fallback(handler_404);

    let app = router.with_state(shared_pool);

    // run it with hyper on localhost:3000
    println!("Binding server port {}", &server_port);

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

// Build a client to the s3 instance
async fn client_to_s3() -> Client {
    let key_id = env::var("MINIO_ACCESS_KEY_ID").expect("No S3 key id !");
    let secret_key = env::var("MINIO_SECRET_ACCESS_KEY").expect("No secret access key !");
    let url = env::var("MINIO_URL").expect("No S3 URL !");
    let bucket_name = env::var("MINIO_BUCKET_NAME").expect("No bucket name !");

    let cred = Credentials::new(key_id, secret_key, None, None, "loaded-from-custom-env");

    let s3_config = aws_sdk_s3::config::Builder::new()
        .endpoint_url(url)
        .credentials_provider(cred)
        .region(Region::new("eu-central-1"))
        .force_path_style(true) // apply bucketname as path param instead of pre-domain
        .build();

    let client = aws_sdk_s3::Client::from_conf(s3_config);

    //FIXME handle already existing bucket
    let result = client.create_bucket().bucket(bucket_name).send().await;
    match result {
        Ok(_) => println!("Bucket created !"),
        Err(err) => println!("Error with bucket creation: {:?}", err),
    }

    client
}
