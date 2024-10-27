use std::path::Path;
use std::process;
use std::sync::Arc;
use log::warn;
use multitool_hg::database::postgres::new_postgres_pool;
use multitool_hg::logger::tracer_logger::new_tracer_logger;
use multitool_hg::rediska::client::Rediska;
use tokio::signal;
use crate::app::AppState;
use crate::cli::Cli;

mod cli;
mod config;
mod api;
mod structs;
mod app;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Fatal error occurred: {}", err);
        process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::new();
    new_tracer_logger(cli.log_level);

    let config = config::AuthConfig::new(Path::new(&cli.config)).expect("Failed to load config");

    let redis_pool = Rediska::new(config.redis).await.expect("Failed to create Redis poll");
    let database_pool = new_postgres_pool(config.database).await.expect("Failed to create Postgres pool");
    let app_state = Arc::new(AppState::new(database_pool, redis_pool));

    let app = api::create_router(app_state);

    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let server = async {
        axum::serve(listener, app).await.unwrap();
    };

    let shutdown_signal = async {
        signal::ctrl_c().await.expect("Failed to install CTRL+C signal handler");
        warn!("Receive stop signal. Start shutdown process...");
    };

    tokio::select! {
        _ = server => {
            warn!("The server has terminated its work.");
        }
        _ = shutdown_signal => {
            warn!("Graceful shutdown initiated...");
        }
    }

    Ok(())
}
