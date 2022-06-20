use std::{net::SocketAddr, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

use axum::{
    routing::get,
    Router, Extension,
};

#[tokio::main]
async fn main() {
    let counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    let port = std::env::var("PORT").map(|val| val.parse::<u16>().unwrap()).unwrap_or(3000);

    let app = Router::new()
        .route("/pingpong", get(pingpong))
        .route("/count", get(count))
        .layer(Extension(counter));

    let addr = SocketAddr::from(([0,0,0,0], port));

    println!("Started at port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn pingpong(Extension(counter): Extension<Arc<AtomicUsize>>) -> String {
    let value = counter.fetch_add(1, Ordering::SeqCst);

    format!("pong {}", value)
}

async fn count(Extension(counter): Extension<Arc<AtomicUsize>>) -> String {
    let value = counter.load(Ordering::Relaxed);

    format!("{}", value)
}
