# Blog Engine

## Project Overview

This project is a Rust-based blog engine that aggregates content from multiple sources and presents it in a cohesive, markdown-friendly format. Designed with a functional backend and a simple UI, this blog engine serves as an educational and practical example of working with Rust frameworks, web scraping, data parsing, and templating.

## Features

- **Content Aggregation**: Collect top blog posts from sources like Dev.to and Hacker News.
- **Markdown Parsing**: Render blog content in Markdown with HTML styling.
- **Post Listing**: Display articles with titles, excerpts, and links.
- **Categories**: Organize posts by categories (e.g., News, Technology).
- **Basic Templating**: Manage dynamic content layout using Tera.
- **Routing**: Implement navigation between pages.

## Technologies Used

- Rust (using Rocket framework)
- HTML, CSS, JavaScript (for frontend rendering)
- Markdown parsing (via Rust's `pulldown-cmark`)

## Project Setup

### Prerequisites

1. **Rust**: Install Rust through [rustup](https://rustup.rs/).
2. **Project Dependencies**: Check the `[dependencies]` section in `Cargo.toml` for Rust libraries, including `Rocket`, `serde`, `pulldown-cmark`, and `tera`.

### Directory Structure

Here's an overview of the main project directories and files:

```
/Blog-Engine
├── src/
│   ├── main.rs                # Main Rust file for routing and rendering
├── templates/                 # HTML templates directory
│   ├── index.html.tera        # Home page
│   ├── posts.html.tera        # Posts listing page
│   └── category.html.tera     # Category-specific posts page
├── static/css/styles.css      # CSS for styling
├── Rocket.toml                # Rocket configuration file
└── Cargo.toml                 # Project dependencies and Rust settings
```

## Progress Made

### 1. Initial Setup and Dependencies

- Set up the initial Rocket configuration with routing and templating.
- Added dependencies in `Cargo.toml`, including `Rocket`, `serde`, `pulldown-cmark`, and `tera` for rendering and HTTP requests.
- Fixed `toolchain` to stable Rust version for the project directory.

### 2. HTML Templating

- Created HTML template files using Tera for the `index`, `posts`, and `category` pages.
- Designed a minimal, user-friendly CSS for frontend styling.
- Troubleshooting included fixing path issues for the template directory and CSS files.

### 3. Implementing Content Parsing and Markdown Conversion

- Used `pulldown-cmark` to parse fetched Markdown content and convert it to HTML for rendering.
- Improved error handling within Markdown parsing to provide fallbacks if parsing fails.

### 4. GitHub and CLI Troubleshooting

- Encountered frequent GitHub CLI login prompts, resolved by setting up persistent GitHub authentication with `gh`.
- Installed missing CLI tools using package managers (e.g., `apt install gh` and environment-specific configuration for Git).
- Fixed toolchain management with `rustup override` to streamline CLI usage.

## Known Issues and Challenges

- **Environment Setup**: Required repeated configurations for Rust, including issues with Rust toolchains. Resolved by creating a stable override in Rust.
- **GitHub CLI Authentication**: Frequent login prompts were resolved through persistent authentication using `gh` and environment variables.

## Running the Project

1. **Run the Project**:
   Ensure all dependencies are installed, then run:
   ```bash
   cargo run
   ```

2. **Check Data Fetching**:
   The Rust application will aggregate and display articles according to specified categories.

## Future Enhancements

- **RSS Feed Integration**: Automate article retrieval using RSS feeds instead of web scraping.
- **UI Improvements**: Enhance the CSS for better UX and responsiveness.
- **Error Monitoring**: Implement logs for improved error tracing in production.
- **Caching**: Introduce caching to store articles locally, reducing load times and dependency on external sites.

## Conclusion

This blog engine project covers essential Rust web development skills, providing a platform for data aggregation, Markdown rendering, and a basic UI. With enhancements, it could evolve into a lightweight, Rust-powered content aggregator similar to Alltop.

--- 

