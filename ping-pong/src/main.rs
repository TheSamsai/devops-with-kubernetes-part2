use std::{net::SocketAddr, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

use axum::{
    routing::get,
    Router, Extension,
};

use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").map(|val| val.parse::<u16>().unwrap()).unwrap_or(3000);

    let postgres_password = std::env::var("POSTGRES_PASSWORD").unwrap_or(String::from("password"));
    let postgres_host = std::env::var("POSTGRES").unwrap_or(String::from("localhost/pingpong"));

    let postgres = format!("postgres://postgres:{}@{}", postgres_password, postgres_host);

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres).await.expect("Failed to connect to Postgres!");

    initialize_database_table(&pool).await;

    let app = Router::new()
        .route("/pingpong", get(pingpong))
        .route("/count", get(count))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0,0,0,0], port));

    println!("Started at port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn pingpong(Extension(pool): Extension<PgPool>) -> String {
    let value = fetch_counter(&pool).await;

    increment_counter(&pool).await;

    format!("pong {}", value)
}

async fn count(Extension(pool): Extension<PgPool>) -> String {
    let value = fetch_counter(&pool).await;
    
    format!("{}", value)
}

async fn initialize_database_table(pool: &PgPool) {
    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS pingpongs
(
    id      INTEGER PRIMARY KEY,
    counter INTEGER
);
        "#
    )
    .execute(pool)
    .await.expect("Failed to create database table");

    sqlx::query(
        r#"
    INSERT INTO pingpongs (id, counter) VALUES (0, 0) ON CONFLICT DO NOTHING;
        "#
    )
    .execute(pool)
    .await.expect("Failed to inititalize table");
}

async fn fetch_counter(pool: &PgPool) -> i32 {
    let row: (i32,) = sqlx::query_as("SELECT counter FROM pingpongs WHERE id = 0")
        .fetch_one(pool)
        .await.expect("Failed to fetch data from DB");

    row.0
}

async fn increment_counter(pool: &PgPool) {
    let row: (i32,) = sqlx::query_as("SELECT counter FROM pingpongs WHERE id = 0")
        .fetch_one(pool)
        .await.expect("Failed to fetch data from DB");
       
    sqlx::query("UPDATE pingpongs SET counter = $1 WHERE id = 0")
        .bind(row.0 + 1)
        .execute(pool)
        .await.expect("Failed to update data in DB");
}
