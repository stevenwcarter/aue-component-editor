use log::*;

// use aue_component_editor::routes::app;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(targetenv = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Edit {
        #[arg(short, long)]
        list: bool,
    },
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let env_result = dotenvy::dotenv();
    match env_result {
        Err(_) => println!(".env file did not exist, ignoring"),
        Ok(_) => println!("Loaded environment"),
    }

    tracing_subscriber::fmt::init();

    let args = Cli::parse();

    info!("args: {:#?}", args);

    let component_models = fs::read_to_string("component-models.json")
        .context("could not read component-models.json")?;
    let component_filters = fs::read_to_string("component-filters.json")
        .context("could not read component-filters.json")?;
    let component_definition = fs::read_to_string("component-definition.json")
        .context("could not read component-definition.json")?;

    let component_models =
        serde_json::from_str(&component_models).context("could not parse component-models.json")?;
    let component_filters = serde_json::from_str(&component_filters)
        .context("could not parse component-filters.json")?;
    let component_definition = serde_json::from_str(&component_definition)
        .context("could not parse component-definition.json")?;

    // TODO create state
    // TODO create write function for state
    // TODO add clap parsing for different functions

    Ok(())

    // let port = 7070;

    // // pretty_env_logger::init();

    // let app = app();

    // let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
    //     .await
    //     .unwrap();
    // info!("listener set up");
    // axum::serve(listener, app)
    //     .with_graceful_shutdown(async move {
    //         tokio::signal::ctrl_c()
    //             .await
    //             .expect("failed to listen for shutdown signal");
    //     })
    //     .await
    //     .expect("Could not keep server open");
}
