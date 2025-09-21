pub mod models;
pub mod order_dtos;
pub mod errors;
pub mod db;
pub mod routes;


use crate::db::Db;
use std::{collections::HashMap, env, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(RwLock::new(HashMap::new()));
    let app = routes::app(db);

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    if env::var("DEV_LOGGING").unwrap_or_else(|_| "0".into()) == "1" {
        tracing_subscriber::fmt()
            .with_env_filter("tower_http=trace,info")
            .init();
    }
    
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().expect("Invalid host/port");
    println!("Server running at http://{}", addr);

    // New Axum 0.7 style
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

