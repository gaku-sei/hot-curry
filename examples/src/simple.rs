#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::{env, path::Path};

use anyhow::Result;
use clap::Parser;
use hot_curry::FullTemplate;

#[derive(Debug, Parser)]
struct Args;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let _args = Args::parse();

    println!("{}", env!("CARGO_MANIFEST_DIR"));

    let mut full_template = FullTemplate::from_config_path(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/hot_curry.toml"),
    )
    .await?;

    full_template.generate_files()?;

    Ok(())
}
