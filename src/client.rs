use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use rss::Channel;

use crate::article::Article;
use crate::error::Error;

/// Client for fetching Medium articles via RSS feeds.
///
/// # Example
///
/// ```rust,no_run
/// use medium_api::Client;
///
/// # async fn example() -> Result<(), medium_api::Error> {
/// let client = Client::new();
/// let articles = client.get_user_articles("username").await?;
///
/// for article in articles {
///     println!("{}: {}", article.title, article.url);
/// }
/// # Ok(())
/// # }
/// ```
pub struct Client {
    http: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Creates a new Medium client.
    #[must_use]
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("medium-api/0.1.0"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("failed to build HTTP client");

        Self { http }
    }

    /// Fetches articles from a user's Medium feed.
    ///
    /// # Arguments
    ///
    /// * `username` - The Medium username (with or without the @ prefix)
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or the RSS feed cannot be parsed.
    pub async fn get_user_articles(&self, username: &str) -> Result<Vec<Article>, Error> {
        let username = username.trim_start_matches('@');
        let url = format!("https://medium.com/feed/@{username}");
        self.fetch_feed(&url).await
    }

    /// Fetches articles from a Medium publication.
    ///
    /// # Arguments
    ///
    /// * `publication` - The publication name/slug
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or the RSS feed cannot be parsed.
    pub async fn get_publication_articles(&self, publication: &str) -> Result<Vec<Article>, Error> {
        let url = format!("https://medium.com/feed/{publication}");
        self.fetch_feed(&url).await
    }

    async fn fetch_feed(&self, url: &str) -> Result<Vec<Article>, Error> {
        let response = self.http.get(url).send().await?;

        if response.status() == 404 {
            return Err(Error::NotFound(url.to_string()));
        }

        let content = response.bytes().await?;
        let channel = Channel::read_from(&content[..])?;

        let articles = channel
            .items()
            .iter()
            .filter_map(|item| Self::parse_item(item, &channel))
            .collect();

        Ok(articles)
    }

    fn parse_item(item: &rss::Item, channel: &Channel) -> Option<Article> {
        let title = item.title()?.to_string();
        let url = item.link()?.to_string();
        let content_html = item
            .content()
            .or_else(|| item.description())
            .unwrap_or("")
            .to_string();

        // Extract ID from URL (last segment before query params)
        let id = extract_article_id(&url).unwrap_or_else(|| url.clone());

        // Parse publication date
        let published_at = item
            .pub_date()
            .and_then(|d| DateTime::parse_from_rfc2822(d).ok())
            .map(|d| d.with_timezone(&Utc))?;

        // Convert HTML to Markdown
        let content_markdown = html2md::parse_html(&content_html);

        // Extract tags from categories
        let tags: Vec<String> = item.categories().iter().map(|c| c.name().to_string()).collect();

        // Get author from item or fall back to channel title
        let author = item
            .author()
            .or_else(|| {
                item.dublin_core_ext()
                    .and_then(|dc| dc.creators().first().map(String::as_str))
            })
            .map_or_else(|| channel.title().to_string(), ToString::to_string);

        Some(Article {
            id,
            title,
            url,
            published_at,
            content_html,
            content_markdown,
            tags,
            author,
        })
    }
}

/// Extracts the article ID from a Medium URL.
///
/// Medium URLs typically have the format:
/// - `https://medium.com/@user/article-title-abc123def456`
/// - `https://medium.com/publication/article-title-abc123def456`
///
/// The ID is the hex string at the end of the slug.
fn extract_article_id(url: &str) -> Option<String> {
    // Try to parse as URL and get the path
    let parsed = url::Url::parse(url).ok()?;
    let path = parsed.path();

    // Get the last path segment (the article slug)
    let slug = path.rsplit('/').next()?;

    // The ID is typically the last part after the final dash
    // Medium IDs are usually 10-12 hex characters
    if let Some(last_dash) = slug.rfind('-') {
        if let Some(potential_id) = slug.get(last_dash + 1..) {
            // Verify it looks like a Medium ID (hex characters)
            if potential_id.len() >= 10 && potential_id.chars().all(|c| c.is_ascii_hexdigit()) {
                return Some(potential_id.to_string());
            }
        }
    }

    // Fall back to using the full slug as ID
    Some(slug.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_article_id() {
        assert_eq!(
            extract_article_id("https://medium.com/@user/my-article-abc123def456"),
            Some("abc123def456".to_string())
        );

        assert_eq!(
            extract_article_id("https://medium.com/publication/great-post-deadbeef1234"),
            Some("deadbeef1234".to_string())
        );

        // Short ID should fall back to full slug
        assert_eq!(
            extract_article_id("https://medium.com/@user/short-id-abc"),
            Some("short-id-abc".to_string())
        );
    }
}
