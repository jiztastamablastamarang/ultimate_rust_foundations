use axum::response::Html;
use axum::{routing::get, Json, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/json", get(hello_json));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> Html<String> {
    let path = std::path::Path::new("src/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();

    return Html(content);
}

#[derive(serde::Serialize)]
struct HelloJson {
    message: String,
}

async fn hello_json() -> Json<HelloJson> {
    return Json(HelloJson {
        message: "Hello, World!".to_string(),
    });
}
