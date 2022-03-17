#![allow(dead_code, unused_imports, unused_variables)]
#![deny(clippy::unwrap_used)]
mod api_models;
mod handler;
mod route;
mod apis;

use std::net::SocketAddr;
use std::str::FromStr;

use api_models::service::Service;
use axum::extract::Extension;
use db_schema::data::prelude::*;
use db_schema::sea_orm::{prelude::*, ConnectionTrait, Database, Schema};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = std::env::var("HOST").expect("HOST is not set in .env file");
    let port = std::env::var("PORT").expect("PORT is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let app = route::init_router().layer(
        ServiceBuilder::new()
            .layer(CookieManagerLayer::new())
            .layer(Extension(conn))
           
    );

    let server_url = format!("{}:{}", host, port);
    let addr = SocketAddr::from_str(&server_url)?;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
