use anyhow::Result;
use clap::Clap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::Path;
use tera::{Context as TeraContext, Tera};

use crate::config::{Config, StyleSource};
use crate::utils::read_file_to_string;

mod config;
mod utils;

/// Simple program to greet a person
#[derive(Clap, Debug)]
#[clap(name = "hot-curry")]
struct Options {
    /// Path to the configuration file
    #[clap(short, long, default_value = "./hot_curry.toml")]
    config_path: String,
}

fn main() -> Result<()> {
    let Options { config_path } = Options::parse();

    let config_file_base_path = Path::new(config_path.as_str())
        .parent()
        .unwrap_or_else(|| Path::new("./"));

    let config_content = read_file_to_string(config_path.as_str())?;

    let config = toml::from_str::<Config>(config_content.as_str())?;

    let source_content =
        read_file_to_string(config_file_base_path.join(config.source.path.as_str()))?;

    let source = config
        .source
        .type_
        .deserialize_str::<serde_json::Value>(source_content.as_str())?;

    let context = TeraContext::from_value(source)?;

    let style_content = match config.style.source {
        StyleSource::Simple(path) | StyleSource::Path { path } => {
            read_file_to_string(config_file_base_path.join(path.as_str()))?
        }
        StyleSource::Url { url } => reqwest::blocking::get(url)?.text()?,
    };

    let output = Tera::one_off(style_content.as_str(), &context, false)?;

    let output_base_path = config_file_base_path.join(config.output.path.as_str());

    let output_base_path = output_base_path.to_string_lossy();

    config
        .output
        .types
        .par_iter()
        .try_for_each(move |type_| -> Result<()> {
            type_.write(output_base_path.as_ref(), output.as_str())
        })?;

    Ok(())
}
