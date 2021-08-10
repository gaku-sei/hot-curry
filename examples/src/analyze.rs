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

    let full_template = FullTemplate::from_config_path(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/hot_curry.toml"),
    )
    .await?;

    let mut visitor = full_template.visit_renderer();

    let sorted_idents = visitor.sorted_idents();
    if !sorted_idents.is_empty() {
        println!("Used variables:");
        for ident in visitor.sorted_idents() {
            println!("\t- {ident}");
        }
    }

    let sorted_iterable_idents = visitor.sorted_iterable_idents();
    if !sorted_iterable_idents.is_empty() {
        println!("Iterable variables:");
        for ident in visitor.sorted_iterable_idents() {
            println!("\t- {ident}");
        }
    }

    let sorted_optional_idents = visitor.sorted_optional_idents();
    if !sorted_optional_idents.is_empty() {
        println!("Potentially optional variables:");
        for ident in visitor.sorted_optional_idents() {
            println!("\t- {ident}");
        }
    }

    Ok(())
}
