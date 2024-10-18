#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use rocket::fs::{FileServer, relative};

// Struct for blog articles
#[derive(Deserialize, Serialize, Debug, Clone)]
struct BlogArticle {
    id: String,
    title: String,
    url: String,
    excerpt: String,
    tags: Vec<String>,
}

// Function to fetch blog data from Dev.to
fn fetch_devto_blog_data(api_key: &str) -> Result<Vec<BlogArticle>, Box<dyn std::error::Error>> {
    let url = format!("https://dev.to/api/articles?api_key={}", api_key);
    let client = Client::new();
    let response = client.get(&url).send()?;

    if response.status().is_success() {
        let articles: Vec<BlogArticle> = response.json()?;
        Ok(articles)
    } else {
        Err(format!("Error fetching blog data from Dev.to: {}", response.status()).into())
    }
}

// Function to fetch blog data from Hacker News
fn fetch_hacker_news_data() -> Result<Vec<BlogArticle>, Box<dyn std::error::Error>> {
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let client = Client::new();
    let response = client.get(url).send()?;

    if response.status().is_success() {
        let ids: Vec<u32> = response.json()?;
        let mut articles = Vec::new();

        for id in ids.iter().take(5) { // Fetching top 5 stories
            let story_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
            let story_response = client.get(&story_url).send()?;

            if story_response.status().is_success() {
                let story: BlogArticle = story_response.json()?;
                articles.push(story);
            }
        }
        Ok(articles)
    } else {
        Err(format!("Error fetching blog data from Hacker News: {}", response.status()).into())
    }
}

// Fake blog data function
fn fetch_fake_blog_data() -> Vec<BlogArticle> {
    vec![
        BlogArticle {
            id: String::from("1"),
            title: String::from("Fake Blog Post 1"),
            url: String::from("https://example.com/post1"),
            excerpt: String::from("This is a description for fake blog post 1."),
            tags: vec![String::from("Tech")],
        },
        BlogArticle {
            id: String::from("2"),
            title: String::from("Fake Blog Post 2"),
            url: String::from("https://example.com/post2"),
            excerpt: String::from("This is a description for fake blog post 2."),
            tags: vec![String::from("Lifestyle")],
        },
    ]
}

// Fetch blog data from all sources
fn fetch_blog_data(api_key: &str) -> Vec<BlogArticle> {
    let devto_articles = fetch_devto_blog_data(api_key).unwrap_or_else(|_| Vec::new());
    let hacker_news_articles = fetch_hacker_news_data().unwrap_or_else(|_| Vec::new());
    
    // Combine articles from both sources
    let mut combined_articles = devto_articles;
    combined_articles.extend(hacker_news_articles);

    if combined_articles.is_empty() {
        fetch_fake_blog_data()
    } else {
        combined_articles
    }
}

#[get("/")]
fn index() -> Template {
    let context = context! {
        title: "Blog Engine",
        message: "This is the home page",
        recent_articles: Vec::<BlogArticle>::new(),
    };
    Template::render("index", &context)
}

#[get("/posts")]
fn list_posts() -> Template {
    let api_key = "YOUR_DEVTO_API_KEY"; // Your Dev.to API Key
    let articles = fetch_blog_data(api_key);

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
