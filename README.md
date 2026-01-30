# medium-api

Rust client for fetching Medium articles via RSS feeds.

## Features

- Fetch articles from user feeds (`@username`)
- Fetch articles from publication feeds
- Automatic HTML to Markdown conversion
- Extract article metadata (title, date, tags, author)

## Quick Start

```rust
use medium_api::Client;

#[tokio::main]
async fn main() -> Result<(), medium_api::Error> {
    let client = Client::new();

    // Fetch from a user's feed
    let articles = client.get_user_articles("username").await?;

    for article in articles {
        println!("{}: {}", article.title, article.url);
    }

    // Fetch from a publication
    let pub_articles = client.get_publication_articles("hackernoon").await?;

    Ok(())
}
```

## Limitations

- **Public posts only** - RSS doesn't include drafts or private posts
- **No authentication** - RSS feeds are public
- **HTML conversion** - Some formatting may be lost (embeds, code highlighting)
- **Feed caching** - Medium caches RSS; new posts may have slight delay

## License

MIT
