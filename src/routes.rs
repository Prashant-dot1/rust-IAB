use axum::{
    extract::{Path, State}, http, routing::{get, post, put}, Json, Router
};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

use crate::{
    db::{self, Db}, errors::ApiError, order_dtos::{CreateOrderDto, OrderResponseDto, UpdateStatusDto}
};

pub fn app(db: Db) -> Router {
    Router::new()
        .route("/orders", post(create).get(list))
        .route("/orders/{id}", get(get_one).delete(delete_one))
        .route("/orders/{id}/status", put(update_status))
        .with_state(db)
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &http::Request<_>, _span: &tracing::Span| {
                    tracing::info!("Incoming: {} {}", request.method(), request.uri());
                })
                .on_response(|response: &http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    tracing::info!("Response: {} (took {:?})", response.status(), latency);
                })
        )
}

async fn create(State(db): State<Db>, Json(payload): Json<CreateOrderDto>) -> Result<Json<OrderResponseDto>, ApiError> {
    let order = db::create_order(db, payload).await?;
    Ok(Json(order))
}

async fn get_one(State(db): State<Db>, Path(id): Path<Uuid>) -> Result<Json<OrderResponseDto>, ApiError> {
    let order = db::get_order(db, id).await?;
    Ok(Json(order))
}

async fn list(State(db): State<Db>) -> Result<Json<Vec<OrderResponseDto>>, ApiError> {
    Ok(Json(db::list_orders(db).await))
}

async fn update_status(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateStatusDto>,
) -> Result<Json<OrderResponseDto>, ApiError> {
    let order = db::update_status(db, id, payload).await?;
    Ok(Json(order))
}

async fn delete_one(State(db): State<Db>, Path(id): Path<Uuid>) -> Result<(), ApiError> {
    db::delete_order(db, id).await?;
    Ok(())
}
