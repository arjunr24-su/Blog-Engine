#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use rocket::fs::{FileServer, relative};
use pulldown_cmark::{Parser, html};
use rocket::http::Status;

// Struct for Ghost Articles
#[derive(Deserialize, Serialize, Debug, Clone)]
struct GhostArticle {
    id: String,
    title: String,
    url: String,
    excerpt: String,
    tags: Vec<String>,
}

// Function to fetch blog data from Ghost
fn fetch_blog_data(api_key: &str, blog_url: &str) -> Result<Vec<GhostArticle>, Box<dyn std::error::Error>> {
    let url = format!("{}/ghost/api/v3/content/posts/?key={}", blog_url, api_key);
    let client = Client::new();
    let response = client.get(&url).send()?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json()?;
        let articles: Vec<GhostArticle> = serde_json::from_value(json["posts"].clone())?;
        Ok(articles)
    } else {
        Err(format!("Error fetching blog data: {}", response.status()).into())
    }
}

// Function to convert Markdown to HTML
fn markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new(markdown);
    html::push_html(&mut html_output, parser);
    html_output
}

// Fake blog data function
fn fetch_fake_blog_data() -> Vec<GhostArticle> {
    vec![
        GhostArticle {
            id: String::from("1"),
            title: String::from("Fake Blog Post 1"),
            url: String::from("https://example.com/post1"),
            excerpt: String::from("This is a description for fake blog post 1."),
            tags: vec![String::from("Tech")],
        },
        GhostArticle {
            id: String::from("2"),
            title: String::from("Fake Blog Post 2"),
            url: String::from("https://example.com/post2"),
            excerpt: String::from("This is a description for fake blog post 2."),
            tags: vec![String::from("Lifestyle")],
        },
    ]
}

#[get("/")]
fn index() -> Template {
    let context = context! {
        title: "Blog Engine",
        message: "This is the home page",
        recent_articles: Vec::<GhostArticle>::new(),
    };
    Template::render("index", &context)
}

#[get("/posts")]
fn list_posts() -> Template {
    let api_key = "YOUR_GHOST_API_KEY"; // Your Ghost API Key
    let blog_url = "YOUR_GHOST_BLOG_URL"; // Your Ghost Blog URL
    let articles = fetch_blog_data(api_key, blog_url).unwrap_or_else(|_| fetch_fake_blog_data());

    let context = context! {
        title: "All Blog Posts",
        articles,
    };

    Template::render("posts", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, list_posts])
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
}
