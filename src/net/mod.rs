pub mod dns;
pub mod http;
pub mod url_input;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid url: {0}")]
    InvalidUrl(String),
    #[error("unsupported scheme (only http): {0}")]
    UnsupportedScheme(String),
    #[error("failed to read hosts file {path}: {source}")]
    HostsFileRead { path: String, source: std::io::Error },
    #[error("dns/hosts resolution failed for {host} (hosts: {path})")]
    ResolveFailed { host: String, path: String },
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("http parse error: {0}")]
    HttpParse(String),
    #[error("other: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
