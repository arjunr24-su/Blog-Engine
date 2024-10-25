#[macro_use]
extern crate rocket;

use reqwest::blocking::Client;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BlogArticle {
    id: String,
    title: String,
    url: String,
    excerpt: String,
    tags: Vec<String>,
    content: String,
    description: Option<String>, // Make description optional
    category: Option<String>,    // Add optional category field
}

// Updated fetch_devto_blog_data function with better error handling
fn fetch_devto_blog_data(api_key: &str) -> Result<Vec<BlogArticle>, String> {
    let url = format!("https://dev.to/api/articles?api_key={}", api_key);
    let client = Client::new();

    match client.get(&url).send() {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Vec<BlogArticle>>() {
                    Ok(mut articles) => {
                        // Ensure each article has at least one tag
                        for article in &mut articles {
                            if article.tags.is_empty() {
                                article.tags.push(String::from("Uncategorized"));
                            }
                            // Set description from excerpt if not present
                            if article.description.is_none() {
                                article.description = Some(article.excerpt.clone());
                            }
                        }
                        Ok(articles)
                    }
                    Err(e) => Err(format!("Failed to parse Dev.to response: {}", e)),
                }
            } else {
                Err(format!("Dev.to API error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Failed to fetch from Dev.to: {}", e)),
    }
}

// Updated fetch_blog_data function
fn fetch_blog_data(_api_key: &str) -> Vec<BlogArticle> {
    // For now, return fake data to ensure the site works
    vec![
        BlogArticle {
            id: String::from("1"),
            title: String::from("Tech Article 1"),
            url: String::from("https://example.com/tech1"),
            excerpt: String::from("A tech article about programming"),
            tags: vec![String::from("Tech")],
            content: String::from("This is the full content of tech article 1"),
            description: Some(String::from("A tech article about programming")),
            category: Some(String::from("Tech")),
        },
        BlogArticle {
            id: String::from("2"),
            title: String::from("Security Article 1"),
            url: String::from("https://example.com/security1"),
            excerpt: String::from("A security article about cybersecurity"),
            tags: vec![String::from("Security")],
            content: String::from("This is the full content of security article 1"),
            description: Some(String::from("A security article about cybersecurity")),
            category: Some(String::from("Security")),
        },
    ]
}

#[get("/")]
fn index() -> Template {
    let articles = fetch_blog_data("dummy_key");
    Template::render(
        "index",
        context! {
            title: "Blog Engine",
            message: "Welcome to the Blog Engine",
            recent_articles: articles,
        },
    )
}

#[get("/posts")]
fn list_posts() -> Template {
    let articles = fetch_blog_data("dummy_key");
    Template::render(
        "posts",
        context! {
            title: "All Blog Posts",
            articles: articles,
        },
    )
}

#[get("/category/<tag>")]
fn posts_by_category(tag: String) -> Template {
    let all_articles = fetch_blog_data("dummy_key");

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, list_posts, posts_by_category])
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
}
