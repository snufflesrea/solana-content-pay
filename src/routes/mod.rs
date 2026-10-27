mod health;
mod products;
mod paid;

use axum::{Router, routing::get};
use std::sync::Arc;
use crate::shared::config::Config;

pub fn create_router() -> Router<Arc<Config>> {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/products", get(products::products_endpoint))
        .route("/product/{product_id}", get(paid::paid_endpoint))
}
