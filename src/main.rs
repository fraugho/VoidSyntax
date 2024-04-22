use axum::{
    extract::{multipart, Multipart}, http::StatusCode,
    response::*, routing::{get, get_service, post},
    Json, Router
};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use std::fs::read_to_string;
use std::process::Command;

use serde::{Deserialize, Serialize};

use rand::prelude::*;

use names::{Generator, Name};

mod structs;
use structs::*;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(main_page))
        .route("/file_upload", post(file_upload))
        .route("/login", post(login))
        .route("/create_login", post(create_login))
        .route("/submit", post(submit))
        .nest_service("/static", get_service(ServeDir::new("static")).handle_error(|error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }))
    .layer(CorsLayer::permissive());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn main_page() -> Result<Html<String>, (StatusCode, String)> {
    let path = "static/HTML/home_page.html";
    match read_to_string(path) {
        Ok(content) => Ok(Html(content)),
        Err(err) => {
            println!("Failed to read login_page HTML: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".into()))
        }
    }
}

async fn submit(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get next field!")
    {

        let file_name = match field.file_name() {
            Some(file_name) => file_name,
            None => continue,
        };

        // Create a path for the soon-to-be file
        let file_path = format!("static/uploads/{}", file_name);

        // Unwrap the incoming bytes
        let data = field.bytes()
            .await.unwrap();

        // Open a handle to the file
        let mut file_handle = File::create(file_path)
            .await
            .expect("Failed to open file handle!");

        // Write the incoming data to the handle
        file_handle.write_all(&data)
            .await
            .expect("Failed to write to file!");
    }

    Command::new("docker")
        .args(["run", "--rm", "-v", "{host_path}:{container_path}", "{image_name}", "cargo", "run"])
        .output()
        .expect("failed to execute process");

}

async fn login(Form(login_form): Form<LoginForm>) -> impl IntoResponse {
    (StatusCode::OK, "wuba wuba dub dub")
}

async fn create_login(Form(login_form): Form<LoginForm>) -> impl IntoResponse {
    (StatusCode::OK, "Doh!!")
}

async fn file_upload(mut multipart: Multipart) {
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get next field!")
    {

        let file_name = match field.file_name() {
            Some(file_name) => file_name,
            None => continue,
        };

        // Create a path for the soon-to-be file
        let file_path = format!("static/uploads/{}", file_name);

        // Unwrap the incoming bytes
        let data = field.bytes()
            .await.unwrap();

        // Open a handle to the file
        let mut file_handle = File::create(file_path)
            .await
            .expect("Failed to open file handle!");

        // Write the incoming data to the handle
        file_handle.write_all(&data)
            .await
            .expect("Failed to write to file!");
    }
}
