#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{Template, context}; // Updated to include `context` from `rocket_dyn_templates`
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use rocket::fs::{FileServer, relative};
use std::env;

// Struct for blog articles
#[derive(Deserialize, Serialize, Debug, Clone)]
struct BlogArticle {
    id: String,
    title: String,
    url: String,
    excerpt: String,
    tags: Vec<String>,
    content: String, // Field for Markdown content
}

// Function to fetch blog data from Dev.to
fn fetch_devto_blog_data(api_key: &str) -> Result<Vec<BlogArticle>, String> {
    let url = format!("https://dev.to/api/articles?api_key={}", api_key);
    let client = Client::new();
    let response = client.get(&url).send().map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let articles: Vec<BlogArticle> = response.json().map_err(|e| e.to_string())?;
        Ok(articles)
    } else {
        Err(format!("Error fetching blog data from Dev.to: {}", response.status()))
    }
}

// Function to fetch blog data from Hacker News
fn fetch_hacker_news_data() -> Result<Vec<BlogArticle>, String> {
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let client = Client::new();
    let response = client.get(url).send().map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let ids: Vec<u32> = response.json().map_err(|e| e.to_string())?;
        let mut articles = Vec::new();

        // Could be parallelized for optimization
        for id in ids.iter().take(5) { // Fetching top 5 stories
            let story_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
            let story_response = client.get(&story_url).send().map_err(|e| e.to_string())?;

            if story_response.status().is_success() {
                let story: BlogArticle = story_response.json().map_err(|e| e.to_string())?;
                articles.push(story);
            }
        }
        Ok(articles)
    } else {
        Err(format!("Error fetching blog data from Hacker News: {}", response.status()))
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
            content: String::from("This is **Markdown** content for post 1."),
        },
        BlogArticle {
            id: String::from("2"),
            title: String::from("Fake Blog Post 2"),
            url: String::from("https://example.com/post2"),
            excerpt: String::from("This is a description for fake blog post 2."),
            tags: vec![String::from("Lifestyle")],
            content: String::from("This is _Markdown_ content for post 2."),
        },
    ]
}

// Fetch blog data from all sources
fn fetch_blog_data(api_key: &str) -> Result<Vec<BlogArticle>, String> {
    let devto_articles = if let Ok(articles) = fetch_devto_blog_data(api_key) {
        articles
    } else {
        eprintln!("Failed to fetch Dev.to articles.");
        Vec::new()
    };
    
    let hacker_news_articles = if let Ok(articles) = fetch_hacker_news_data() {
        articles
    } else {
        eprintln!("Failed to fetch Hacker News articles.");
        Vec::new()
    };

    let mut combined_articles = devto_articles;
    combined_articles.extend(hacker_news_articles);

    if combined_articles.is_empty() {
        Ok(fetch_fake_blog_data())
    } else {
        Ok(combined_articles)
    }
}

// Function to render Markdown to HTML
fn render_markdown(content: &str) -> String {
    let mut html_output = String::new();
    let parser = pulldown_cmark::Parser::new(content);
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
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
    let api_key = env::var("wr97LShzGaQBpA48BhnkuFwF").expect("Dev.to API key not set");
    let articles_result = fetch_blog_data(&api_key);

    let articles = match articles_result {
        Ok(mut articles) => {
            // Convert Markdown content to HTML for each article
            for article in &mut articles {
                article.content = render_markdown(&article.content);
            }
            articles
        },
        Err(err) => {
            eprintln!("Error fetching posts: {}", err);
            Vec::<BlogArticle>::new() // Return empty Vec on error
        }
    };

    let context = context! {
        title: "All Blog Posts",
        articles,
    };

    Template::render("posts", &context)
}

#[get("/category/<tag>")]
fn posts_by_category(tag: String) -> Template {
    let api_key = env::var("wr97LShzGaQBpA48BhnkuFwF").expect("Dev.to API key not set");
    let articles_result = fetch_blog_data(&api_key);

    let filtered_articles = match articles_result {
        Ok(articles) => {
            articles.into_iter()
                .filter(|article| article.tags.contains(&tag))
                .collect()
        },
        Err(err) => {
            eprintln!("Error fetching posts by category: {}", err);
            Vec::<BlogArticle>::new() // Return empty Vec on error
        }
    };

    let context = context! {
        title: format!("Posts in Category: {}", tag),
        articles: filtered_articles,
    };

    Template::render("category", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, list_posts, posts_by_category])
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
}
