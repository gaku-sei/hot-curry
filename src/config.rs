use anyhow::{anyhow, Context, Result};
use headless_chrome::{protocol::page::PrintToPdfOptions, Browser};
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use std::{
    ffi::OsStr,
    fmt::{self, Formatter},
    fs::File,
    io::Write,
    path::Path,
};
use url::Url;

#[derive(Debug)]
pub enum SourceType {
    Yaml,
    Json,
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

#[derive(Debug)]
pub struct SourceFile {
    pub path: String,
    pub type_: SourceType,
}

impl SourceFile {
    fn deserialize<'de, D>(deserializer: D) -> Result<SourceFile, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(SourceFileVisitor)
    }
}

pub struct SourceFileVisitor;

impl<'de> de::Visitor<'de> for SourceFileVisitor {
    type Value = SourceFile;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "a valid path with one of the following extentions: json, toml, or yml/yaml"
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let path = Path::new(s);

        let extension = path
            .extension()
            .and_then(OsStr::to_str)
            .ok_or_else(|| de::Error::invalid_value(Unexpected::Str(s), &self))?;

        let source_type = match extension {
            "json" => Ok(SourceType::Json),
            "toml" => Ok(SourceType::Toml),
            "yml" | "yaml" => Ok(SourceType::Yaml),
            _ => Err(de::Error::invalid_value(Unexpected::Str(s), &self)),
        }?;

        Ok(SourceFile {
            path: path.to_string_lossy().into_owned(),
            type_: source_type,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Source {
    #[serde(rename = "path", deserialize_with = "SourceFile::deserialize")]
    pub file: SourceFile,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TemplateSource {
    Simple(String),
    Path { path: String },
    Url { url: Url },
}

#[derive(Debug, Deserialize)]
pub struct Template {
    pub source: TemplateSource,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub path: String,
    pub types: Vec<OutputType>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub output: Output,
    pub source: Source,
    pub template: Template,
}
