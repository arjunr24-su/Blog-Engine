#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BlogArticle {
    id: String,
    title: String,
    url: String,
    excerpt: String,
    tags: Vec<String>,
    content: String,
    description: Option<String>,
    category: Option<String>,
}

// Python-based scraper function
async fn fetch_python_scraped_data() -> Vec<BlogArticle> {
    let output = Command::new("python3")
        .arg("scraper.py")
        .output()
        .expect("Failed to execute scraper");

    if output.status.success() {
        let json_str = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(&json_str).unwrap_or_else(|_| Vec::new())
    } else {
        eprintln!("Python scraper error: {:?}", output.stderr);
        Vec::new()
    }
}

// Fetch data from Dev.to API using the given API key
async fn fetch_devto_blog_data(api_key: &str) -> Result<Vec<BlogArticle>, String> {
    let url = format!("https://dev.to/api/articles?api_key={}", api_key);
    let client = Client::new(); // Use async client

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    if response.status().is_success() {
        let mut articles: Vec<BlogArticle> = response.json().await.map_err(|e| e.to_string())?;
        
        for article in &mut articles {
            if article.tags.is_empty() {
                article.tags.push(String::from("Uncategorized"));
            }
            if article.description.is_none() {
                article.description = Some(article.excerpt.clone());
            }
        }
        Ok(articles)
    } else {
        Err(format!("Dev.to API error: {}", response.status()))
    }
}

// Function to retrieve blog data using API key or Python scraper
async fn fetch_blog_data() -> Vec<BlogArticle> {
    let api_key = "44h9fR1BfEW98AUVrkNJYVbd"; // Replace with your actual API key
    
    // Try to fetch from Dev.to, fallback to the Python scraper
    if let Ok(devto_articles) = fetch_devto_blog_data(api_key).await {
        devto_articles
    } else {
        fetch_python_scraped_data().await
    }
}

// Routes

#[get("/")]
async fn index() -> Template {
    let articles = fetch_blog_data().await; // Await the data fetch
    Template::render(
        "index", // Ensure this matches your template file name
        context! {
            title: "Blog Engine",
            message: "Welcome to the Blog Engine",
            recent_articles: articles,
        },
    )
}

#[get("/posts")]
async fn list_posts() -> Template {
    let articles = fetch_blog_data().await; // Await the data fetch
    Template::render(
        "posts",
        context! {
            title: "All Blog Posts",
            articles: articles,
        },
    )
}

#[get("/category/<tag>")]
async fn posts_by_category(tag: String) -> Template {
    let all_articles = fetch_blog_data().await; // Await the data fetch

    // Filter articles by tag (case-insensitive)
    let filtered_articles: Vec<BlogArticle> = all_articles
        .into_iter()
        .filter(|article| {
            article
                .tags
                .iter()
                .any(|t| t.to_lowercase() == tag.to_lowercase())
        })
        .collect();

    Template::render(
        "category",
        context! {
            title: format!("Posts in Category: {}", tag),
            articles: filtered_articles,
            category_description: format!("All articles in the {} category", tag),
        },
    )
}

// Launch the Rocket application
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, list_posts, posts_by_category]) // Include other routes as needed
        .attach(Template::fairing()) // Ensure template fairing is attached
        .mount("/static", FileServer::from(relative!("static"))) // Serve static files
}

