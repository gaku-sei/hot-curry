#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use filters::markdown;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tera::{Context, Template, Tera};
use tera_visitor::VisitorMut;
use types::{Config, TemplateSource};

pub use crate::errors::{Error, Result};
use crate::visitor::TeraVariableVisitor;

pub mod errors;
pub mod filters;
pub mod types;
pub mod visitor;

pub struct FullTemplate {
    tera: Tera,
    template: Template,
    context: Context,
    config: Config,
    template_content: String,
    config_file_base_path: PathBuf,
}

impl FullTemplate {
    fn new(
        tera: Tera,
        template: Template,
        context: Context,
        config: Config,
        template_content: impl Into<String>,
        config_file_base_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            tera,
            template,
            context,
            config,
            template_content: template_content.into(),
            config_file_base_path: config_file_base_path.into(),
        }
    }

    pub async fn from_config_path(config_path: impl AsRef<Path>) -> Result<Self> {
        let config_path = config_path.as_ref();

        let config_file_base_path = config_path.parent().unwrap_or_else(|| Path::new("./"));

        let config_content = fs::read_to_string(config_path)?;

        let config = toml::from_str::<Config>(&config_content)?;

        let source_content =
            fs::read_to_string(config_file_base_path.join(&config.source.file.path))?;

        let context = config
            .source
            .file
            .type_
            .to_value(source_content)?
            .into_tera_context()?;

        let template_content = match &config.template.source {
            TemplateSource::Simple(path) | TemplateSource::Path { path } => {
                fs::read_to_string(config_file_base_path.join(path))?
            }
            TemplateSource::Url { url } => reqwest::get(url.as_str()).await?.text().await?,
        };

        let template = Template::new("", None, &template_content)?;

        let mut tera = Tera::new(&config_file_base_path.join("*").to_string_lossy())?;

        tera.autoescape_on(Vec::new());

        tera.register_filter("markdown", markdown);

        Ok(Self::new(
            tera,
            template,
            context,
            config,
            template_content,
            config_file_base_path,
        ))
    }

    pub fn generate_files(&mut self) -> Result<()> {
        let output = self
            .tera
            .render_str(&self.template_content, &self.context)?;

        let output_base_path = self.config_file_base_path().join(&self.config.output.path);

        let output_base_path = output_base_path.to_string_lossy();

        self.config
            .output
            .types
            .par_iter()
            .try_for_each(|type_| type_.write(&output_base_path, &output))?;

        Ok(())
    }

    #[must_use]
    pub fn visit_renderer(&self) -> TeraVariableVisitor {
        let mut visitor = TeraVariableVisitor::new();

        visitor.visit_ast_mut(&self.template.ast);

        visitor
    }

    #[must_use]
    pub fn tera(&self) -> &Tera {
        &self.tera
    }

    #[must_use]
    pub fn template(&self) -> &Template {
        &self.template
    }

    #[must_use]
    pub fn context(&self) -> &Context {
        &self.context
    }

    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }

    #[must_use]
    pub fn template_content(&self) -> &str {
        &self.template_content
    }

    #[must_use]
    pub fn config_file_base_path(&self) -> &Path {
        &self.config_file_base_path
    }
}
