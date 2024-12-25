use thiserror::Error;

#[derive(Error, Debug)]
pub enum DnsError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parsing Error: {0}")]
    Parse(String),

    #[error("Resolution Error: {0}")]
    Resolution(String),

    #[error("serialization v: {0}")]
    Serialization(String),

    #[error("Unknown Error")]
    Unknown,
}
