#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use hot_curry::FullTemplate;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
enum Subcommands {
    /// Will automatically analyze the template used by the CV and display all the expected variables,
    /// the iterable ones, and the potentially optional ones.
    Analyze,
}

#[derive(Debug, Parser)]
#[clap(name = "hot-curry")]
struct Args {
    /// Path to the configuration file
    #[clap(short, long, default_value = "./hot_curry.toml")]
    config_path: PathBuf,
    #[clap(subcommand)]
    subcommand: Option<Subcommands>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let mut full_template = FullTemplate::from_config_path(args.config_path).await?;

    if let Some(Subcommands::Analyze) = args.subcommand {
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
    } else {
        full_template.generate_files()?;
    }

    Ok(())
}
