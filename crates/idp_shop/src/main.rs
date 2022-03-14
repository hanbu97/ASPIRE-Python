#![allow(dead_code, unused_variables)]
mod api_models;
mod app_context;
mod db_models;
// TODO use db_schema crate?
#[rustfmt::skip]
mod db_schema_codegen;
mod config;
mod handler;
mod route;

fn init_logger() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn init_axum_app(config: &config::Config) -> axum::Router {
    route::init_router()
        .layer(axum::extract::Extension(
            app_context::AppContext::new(config).await,
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}

#[derive(clap::Parser)]
struct Args {
    #[clap(short, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/../../config.toml"), value_name = "PATH")]
    config_file: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let args = <Args as clap::StructOpt>::parse();
    let config = config::Config::read_config_file(args.config_file);
    init_logger();

    axum::Server::bind(&config.api_server.listen_addr())
        .serve(init_axum_app(&config).await.into_make_service())
        .await
        .unwrap();
}
