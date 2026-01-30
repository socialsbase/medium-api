//! # medium-api
//!
//! Rust client for fetching Medium articles via RSS feeds.
//!
//! This crate provides a simple interface to fetch articles from Medium users
//! and publications using their public RSS feeds.
//!
//! ## Features
//!
//! - Fetch articles from user feeds (`@username`)
//! - Fetch articles from publication feeds
//! - Automatic HTML to Markdown conversion
//! - Extract article metadata (title, date, tags, author)
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use medium_api::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), medium_api::Error> {
//!     let client = Client::new();
//!
//!     // Fetch from a user's feed
//!     let articles = client.get_user_articles("username").await?;
//!
//!     for article in articles {
//!         println!("{}: {}", article.title, article.url);
//!     }
//!
//!     // Fetch from a publication
//!     let pub_articles = client.get_publication_articles("hackernoon").await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Limitations
//!
//! - Only public posts are available via RSS
//! - No authentication support (RSS feeds are public)
//! - Some HTML formatting may be lost in conversion (embeds, code highlighting)
//! - Medium caches RSS feeds; new posts may have a slight delay

mod article;
mod client;
mod error;

pub use article::Article;
pub use client::Client;
pub use error::Error;
