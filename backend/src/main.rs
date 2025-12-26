use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use rustls::crypto::CryptoProvider;
use rustls::crypto::ring::default_provider as ring_provider;

mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    let app_state = Arc::new(state::AppState::new().await);

    // println!("{:#?}", app_state.config);

    let axum_backend_port = app_state.config.server.port;

    let axum_socket_address = get_axum_socket_address(Arc::clone(&app_state));

    let rustls_config = get_tls_config().await;

    let router: Router = routes::get_router(app_state);

    tracing::info!(
        "Axum listening on {}, Connect using https://localhost:{}",
        &axum_socket_address,
        &axum_backend_port
    );

    axum_server::bind_rustls(axum_socket_address, rustls_config)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn get_axum_socket_address(app_state: Arc<state::AppState>) -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], app_state.config.server.port))
}

async fn get_tls_config() -> RustlsConfig {
    CryptoProvider::install_default(ring_provider()).expect("Failed to set crypto provider");

    RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("localhost.crt"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("localhost.key"),
    )
    .await
    .unwrap()
}
