#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Toml deserialization error: {0}")]
    TomlDeserialization(#[from] toml::de::Error),

    #[error("Yaml error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Headless browser error: {0}")]
    HeadlessBrowser(String),

    #[error("Url error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Tera error: {0}")]
    Tera(#[from] tera::Error),

    #[error("Http request error: {0}")]
    HttpRequest(#[from] reqwest::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
