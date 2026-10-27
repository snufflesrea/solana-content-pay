use axum::Json;
use serde_json::{json, Value};

pub async fn products_endpoint() -> Json<Value> {
    log::info!("Received request to /api/products");
    Json(json!({
        "message": "List of products available : ",
        "products": 
        "1. Solana Audit Checklist
        2. Solana Program Security Guidelines
        3. Protocol Audit Report
        ",
    }))
}
