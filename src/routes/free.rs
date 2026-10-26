use axum::Json;
use serde_json::{json, Value};

pub async fn free_endpoint() -> Json<Value> {
    log::info!("Received request to /api/free-course");
    Json(json!({
        "message": "This is a free course endpoint - no payment required!",
        "data": "Free course content goes here. Enjoy your learning experience!",
    }))
}
