use std::{net::SocketAddr, time::Instant, sync::Arc};

use axum::{
    routing::{get, get_service},
    Router, response::{IntoResponse, Html, Redirect}, http::StatusCode, Extension, extract::Form, Json,
};

use serde::Deserialize;
use tower_http::services::ServeFile;

use tokio::{process::Command, sync::Mutex};

use tera::Tera;
use tera::Context;

type ImageAge = Arc<Mutex<Instant>>;
type ImageStorage = Arc<String>;
type TodoStorage = Arc<Mutex<Vec<String>>>;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").map(|val| val.parse::<u16>().unwrap()).unwrap_or(3000);
    let image_storage = Arc::new(std::env::var("IMAGE_DIR").unwrap_or(String::from("./image")));

    let image_age: ImageAge = Arc::new(Mutex::new(Instant::now()));
    download_image_of_the_day(image_storage.clone()).await;

    let todo_storage = Arc::new(Mutex::new(Vec::<String>::new()));

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let image_storage_path: String = image_storage.to_string();

    let app = Router::new()
        .route("/image", get_service(ServeFile::new(format!("{}/image.jpg", image_storage_path))).handle_error(handle_error))
        .route("/todos", get(get_todos).post(post_todo))
        .route("/", get(index_page).post(post_todo_form))
        .layer(Extension(image_storage))
        .layer(Extension(image_age))
        .layer(Extension(todo_storage))
        .layer(Extension(tera));

    let addr = SocketAddr::from(([0,0,0,0], port));

    println!("Started at port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_page(
    Extension(image_age_state): Extension<ImageAge>,
    Extension(image_storage): Extension<ImageStorage>,
    Extension(todo_storage): Extension<TodoStorage>,
    Extension(tera): Extension<Tera>
) -> Html<String> {
    check_and_download_image_of_the_day(image_age_state, image_storage).await;

    let todos = todo_storage.lock().await;

    let mut context = Context::new();

    context.insert("todos", &*todos);

    Html(tera.render("index.html", &context).unwrap())
}

#[derive(Deserialize, Debug)]
struct TodoInput {
    todo: String
}

async fn post_todo_form(
    Form(input): Form<TodoInput>,
    Extension(todo_storage): Extension<TodoStorage>
) -> Redirect {
    let mut todos = todo_storage.lock().await;

    todos.push(input.todo);

    Redirect::to("/")
}

async fn get_todos(
    Extension(todo_storage): Extension<TodoStorage>
) -> Json<Vec<String>> {
    let mut todos = todo_storage.lock().await;

    return Json(todos.clone());
}

async fn post_todo(
    Json(payload): Json<TodoInput>,
    Extension(todo_storage): Extension<TodoStorage>
) -> Json<Vec<String>> {
    let mut todos = todo_storage.lock().await;

    todos.push(payload.todo);

    return Json(todos.clone());
}

async fn check_and_download_image_of_the_day(image_age: ImageAge, image_storage: ImageStorage) {
    let mut image_age = image_age.lock().await;

    if Instant::now().duration_since(*image_age).as_secs() > 24 * 60 * 60 {
        download_image_of_the_day(image_storage).await;
        *image_age = Instant::now();
    }
}

async fn download_image_of_the_day(image_dir: Arc<String>) {
    Command::new("wget")
        .arg("https://picsum.photos/1200")
        .arg("-O")
        .arg(format!("{}/image.jpg", image_dir))
        .spawn()
        .expect("Failed to start 'wget'")
        .wait()
        .await
        .expect("'wget' failed to run");
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
