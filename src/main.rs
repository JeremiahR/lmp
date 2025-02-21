use std::fs;

use axum::{
    body::Body,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    serve, Router,
};

async fn serve_wasm() -> impl IntoResponse {
    match fs::read("pkg/lmp_bg.wasm") {
        Ok(bytes) => (
            [(axum::http::header::CONTENT_TYPE, "application/wasm")],
            bytes,
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load WASM").into_response(),
    }
}

#[tokio::main]
async fn main() {
    let index_html = std::fs::read_to_string("index.html").expect("Failed to index file");
    let wasm_js = std::fs::read_to_string("pkg/lmp.js").expect("Failed to wasm file");

    // Build the router
    let app = Router::new()
        .route("/", get(|| async { Html(index_html) }))
        .route(
            "/wasm.js",
            get(|| async {
                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/javascript")
                    .body(Body::from(wasm_js))
                    .unwrap()
            }),
        )
        .route("/lmp_bg.wasm", get(serve_wasm));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
