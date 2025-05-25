#![allow(unused)]

use axum::{Router, response::Html, routing::get};

use hello_rust::greet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(handler))
        .route("/greet", get(greet_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3004").await?;
    println!("🚀 Server running on {:?}", listener.local_addr());

    axum::serve(listener, app).await?;

    Ok(())
}

async fn ready(s: String) -> String {
    s
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello from Axum 0.8.4!</h1><p><a href='/greet'>Visit /greet</a></p>")
}

async fn greet_world() -> String {
    greet("Axum 0.8.4")
}
