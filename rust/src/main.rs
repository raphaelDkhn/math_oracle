mod oracle;

use axum::{extract, routing::post, Json, Router};
use oracle::*;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
struct JsonResult {
    result: MathResponse,
}

async fn math_handler(extract::Json(payload): extract::Json<MathRequest>) -> Json<JsonResult> {
    debug!("Received payload: {:?}", payload);

    // Match against the enum variants directly
    let result = match payload.operation() {
        Operation::Sqrt => (payload.operands[0] as f64).sqrt() as i64,
        Operation::Exp => (payload.operands[0] as f64).exp() as i64,
        _ => panic!("Unsupported or unknown operation"),
    };

    Json(JsonResult {
        result: MathResponse { result },
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/calculate", post(math_handler))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000. Ensure the port is not in use by another process.");
    debug!("Server started on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
