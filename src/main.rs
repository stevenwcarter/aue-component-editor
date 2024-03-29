use log::*;

use aue_component_editor::routes::app;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(targetenv = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let env_result = dotenvy::dotenv();
    match env_result {
        Err(_) => println!(".env file did not exist, ignoring"),
        Ok(_) => println!("Loaded environment"),
    }

    let port = 7070;

    // pretty_env_logger::init();
    tracing_subscriber::fmt::init();

    let app = app();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    info!("listener set up");
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen for shutdown signal");
        })
        .await
        .expect("Could not keep server open");
}
