use thiserror::Error;

/// Errors that can occur when interacting with Medium RSS feeds.
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// RSS feed parsing failed.
    #[error("RSS parsing error: {0}")]
    RssParse(#[from] rss::Error),

    /// The requested user or publication was not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Invalid URL encountered.
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}
