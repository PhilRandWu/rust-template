use std::process::ExitCode;

use clap::Parser;
use {{crate_name}}_core::{CoreError, greet};
use thiserror::Error;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

/// A Rust workspace starter CLI.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name to greet.
    #[arg(short, long, default_value = "world")]
    name: String,
}

#[derive(Debug, Error)]
enum AppError {
    #[error(transparent)]
    Core(#[from] CoreError),
}

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();

    let args = Args::parse();

    match run(&args.name) {
        Ok(message) => {
            info!("command completed");
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            error!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run(name: &str) -> Result<String, AppError> {
    Ok(greet(name)?)
}
