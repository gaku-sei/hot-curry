use anyhow::{anyhow, Context, Result};
use headless_chrome::{protocol::page::PrintToPdfOptions, Browser};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use url::Url;

#[derive(Debug, Deserialize)]
pub enum SourceType {
    #[serde(rename = "yml")]
    Yaml,
    #[serde(rename = "jsonl")]
    Json,
    #[serde(rename = "toml")]
    Toml,
}

impl SourceType {
    pub fn deserialize_str<V>(&self, source: &str) -> Result<V>
    where
        V: for<'de> Deserialize<'de> + Serialize,
    {
        match self {
            SourceType::Json => serde_json::from_str(source).context("Json deserialize error"),
            SourceType::Yaml => serde_yaml::from_str(source).context("Yaml deserialize error"),
            SourceType::Toml => toml::from_str(source).context("Toml deserialize error"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum OutputType {
    #[serde(rename = "pdf")]
    Pdf,
    #[serde(rename = "html")]
    Html,
}

impl OutputType {
    pub fn write(&self, output_base_path: &str, output: &str) -> Result<()> {
        match self {
            OutputType::Html => {
                let mut file = File::create(format!("{}.html", output_base_path))?;

                file.write_all(output.as_bytes())?;
            }
            OutputType::Pdf => {
                let browser = Browser::default().map_err(|_| anyhow!("Browser init error"))?;

                let tab = browser
                    .wait_for_initial_tab()
                    .map_err(|_| anyhow!("Tab init error"))?;

                let url = Url::parse(format!("data:text/html,{}", output).as_str())?;

                tab.navigate_to(url.as_str())
                    .map_err(|_| anyhow!("Couldn't open output html"))?;

                tab.wait_until_navigated()
                    .map_err(|_| anyhow!("Couldn't successfully navigate to html"))?;

                let pdf = tab
                    .print_to_pdf(Some(PrintToPdfOptions {
                        landscape: Some(true),
                        display_header_footer: Some(false),
                        print_background: Some(true),
                        scale: None,
                        paper_width: None,
                        paper_height: None,
                        margin_top: Some(0.),
                        margin_bottom: Some(0.),
                        margin_left: Some(0.),
                        margin_right: Some(0.),
                        page_ranges: None,
                        ignore_invalid_page_ranges: None,
                        header_template: None,
                        footer_template: None,
                        prefer_css_page_size: Some(true),
                    }))
                    .map_err(|_| anyhow!("PDF printing init error"))?;

                let mut file = File::create(format!("{}.pdf", output_base_path))?;

                file.write_all(pdf.as_slice())?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub path: String,
    #[serde(rename = "type")]
    pub type_: SourceType,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StyleSource {
    Simple(String),
    Path { path: String },
    Url { url: Url },
}

#[derive(Debug, Deserialize)]
pub struct Style {
    pub source: StyleSource,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub path: String,
    pub types: Vec<OutputType>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub source: Source,
    pub style: Style,
    pub output: Output,
}
