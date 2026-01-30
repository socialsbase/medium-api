use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A Medium article fetched from an RSS feed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// Unique identifier extracted from the article URL (the GUID).
    pub id: String,
    /// Article title.
    pub title: String,
    /// Full URL to the article on Medium.
    pub url: String,
    /// Publication timestamp.
    pub published_at: DateTime<Utc>,
    /// Original HTML content from the RSS feed.
    pub content_html: String,
    /// Content converted to Markdown.
    pub content_markdown: String,
    /// Tags/categories associated with the article.
    pub tags: Vec<String>,
    /// Author name.
    pub author: String,
}
