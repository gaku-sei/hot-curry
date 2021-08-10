use headless_chrome::{protocol::page::PrintToPdfOptions, Browser};
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};
use std::{
    ffi::OsStr,
    fmt::{self, Formatter},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use tera::Context as TeraContext;
use url::Url;

use crate::errors::{Error, Result};

#[derive(Debug)]
pub enum SourceType {
    Yaml,
    Json,
    Toml,
}

impl SourceType {
    pub fn to_value(&self, source: impl AsRef<str>) -> Result<Value> {
        let value = match self {
            SourceType::Json => serde_json::from_str(source.as_ref())?,
            SourceType::Yaml => serde_yaml::from_str(source.as_ref())?,
            SourceType::Toml => toml::from_str(source.as_ref())?,
        };

        Ok(Value(value))
    }
}

#[derive(Debug)]
pub struct Value(serde_json::Value);

impl Value {
    pub fn into_tera_context(self) -> std::result::Result<TeraContext, tera::Error> {
        TeraContext::from_value(self.0)
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
    pub fn write(&self, output_base_path: impl AsRef<str>, output: impl AsRef<str>) -> Result<()> {
        match self {
            OutputType::Html => {
                let mut file = File::create(format!("{}.html", output_base_path.as_ref()))?;

                file.write_all(output.as_ref().as_bytes())?;
            }
            OutputType::Pdf => {
                let browser = Browser::default()
                    .map_err(|err| Error::HeadlessBrowser(format!("Browser init error: {err}")))?;

                let tab = browser
                    .wait_for_initial_tab()
                    .map_err(|err| Error::HeadlessBrowser(format!("Tab init error: {err}")))?;

                let clean_output = output
                    .as_ref()
                    .replace('%', "%25")
                    .replace('&', "%26")
                    .replace('#', "%23")
                    .replace('"', "%22")
                    .replace('\'', "%27");

                let url = Url::parse(&format!("data:text/html;charset=UTF-8,{clean_output}"))?;

                tab.navigate_to(url.as_str()).map_err(|err| {
                    Error::HeadlessBrowser(format!("Couldn't open output html: {err}"))
                })?;

                tab.wait_until_navigated().map_err(|err| {
                    Error::HeadlessBrowser(format!("Couldn't successfully navigate to html: {err}"))
                })?;

                let pdf = tab
                    .print_to_pdf(Some(PrintToPdfOptions {
                        landscape: Some(false),
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
                    .map_err(|err| {
                        Error::HeadlessBrowser(format!("PDF printing init error: {err}"))
                    })?;

                let mut file = File::create(format!("{}.pdf", output_base_path.as_ref()))?;

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
            "a valid path with one of the following extensions: json, toml, or yml/yaml"
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
    Simple(PathBuf),
    Path { path: PathBuf },
    Url { url: Url },
}

#[derive(Debug, Deserialize)]
pub struct Template {
    pub source: TemplateSource,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub path: PathBuf,
    pub types: Vec<OutputType>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub output: Output,
    pub source: Source,
    pub template: Template,
}
