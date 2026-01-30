# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-31

### Added

- Initial release
- `Client` struct for interacting with Medium RSS feeds
- `get_user_articles()` method to fetch articles from user feeds (`@username`)
- `get_publication_articles()` method to fetch articles from publication feeds
- `Article` struct containing:
  - `id` - Article identifier extracted from URL
  - `title` - Article title
  - `url` - Full article URL
  - `published_at` - Publication date (UTC)
  - `content_html` - Original HTML content
  - `content_markdown` - Automatically converted Markdown content
  - `tags` - Article categories/tags
  - `author` - Article author
- Automatic HTML to Markdown conversion via `html2md`
- Error types for HTTP, RSS parsing, and not found errors
