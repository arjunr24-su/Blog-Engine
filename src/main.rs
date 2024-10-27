#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::Duration;

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

// Fetch articles from Dev.to (using HTML scraping)
async fn scrape_devto_articles() -> Vec<BlogArticle> {
    let url = "https://dev.to";
    let client = Client::new();
    let mut articles = Vec::new();

    match client.get(url).send().await {
        Ok(response) => {
            if let Ok(text) = response.text().await {
                let document = scraper::Html::parse_document(&text);
                let post_selector = scraper::Selector::parse("div.crayons-story").unwrap();
                let title_selector = scraper::Selector::parse("h2.crayons-story__title").unwrap();
                let link_selector = scraper::Selector::parse("a.crayons-story__hidden-navigation").unwrap();
                let excerpt_selector = scraper::Selector::parse("p.crayons-story__description").unwrap();

                for post in document.select(&post_selector).take(5) {
                    let title = post.select(&title_selector).next().map(|e| e.inner_html()).unwrap_or_else(|| "No title".to_string());
                    let link = post.select(&link_selector).next().and_then(|e| e.value().attr("href")).unwrap_or("").to_string();
                    let excerpt = post.select(&excerpt_selector).next().map(|e| e.inner_html()).unwrap_or_else(|| "No excerpt".to_string());

                    articles.push(BlogArticle {
                        id: link.split('/').last().unwrap_or("No ID").to_string(),
                        title,
                        url: format!("{}{}", url, link),
                        excerpt: excerpt.clone(),
                        tags: vec!["Dev.to".to_string()],
                        content: excerpt,
                        description: None,
                        category: None,
                    });
                }
            }
        },
        Err(e) => eprintln!("Failed to fetch Dev.to articles: {}", e),
    }

    articles
}

// Fetch articles from Hacker News API
async fn scrape_hacker_news() -> Vec<BlogArticle> {
    let mut articles = Vec::new();
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to create client");
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";

    match client.get(url).send().await {
        Ok(response) => {
            if let Ok(top_ids) = response.json::<Vec<u32>>().await {
                for &story_id in top_ids.iter().take(5) {
                    let story_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", story_id);
                    if let Ok(story_response) = client.get(&story_url).send().await {
                        if let Ok(story) = story_response.json::<serde_json::Value>().await {
                            let title = story["title"].as_str().unwrap_or("No title").to_string();
                            let url = story["url"].as_str().unwrap_or("").to_string();
                            let text = story["text"].as_str().unwrap_or("No description").to_string();

                            articles.push(BlogArticle {
                                id: story_id.to_string(),
                                title,
                                url,
                                excerpt: text.clone(),
                                tags: vec!["Hacker News".to_string()],
                                content: text,
                                description: None,
                                category: None,
                            });
                        }
                    }
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        },
        Err(e) => eprintln!("Failed to fetch Hacker News articles: {}", e),
    }

    articles
}

// Fetch articles from Reddit
async fn scrape_reddit() -> Vec<BlogArticle> {
    let mut articles = Vec::new();
    let client = Client::new();
    let url = "https://www.reddit.com/r/popular.json"; // Reddit's popular posts

    match client.get(url).header("User-Agent", "rust-app").send().await {
        Ok(response) => {
            if let Ok(data) = response.json::<serde_json::Value>().await {
                if let Some(posts) = data["data"]["children"].as_array() {
                    for post in posts.iter().take(5) {
                        let title = post["data"]["title"].as_str().unwrap_or("No title").to_string();
                        let url = post["data"]["url"].as_str().unwrap_or("").to_string();
                        let excerpt = post["data"]["selftext"].as_str().unwrap_or("No description").to_string();

                        articles.push(BlogArticle {
                            id: post["data"]["id"].as_str().unwrap_or("No ID").to_string(),
                            title,
                            url,
                            excerpt: excerpt.clone(),
                            tags: vec!["Reddit".to_string()],
                            content: excerpt,
                            description: None,
                            category: None,
                        });
                    }
                }
            }
        },
        Err(e) => eprintln!("Failed to fetch Reddit articles: {}", e),
    }

    articles
}

// Combine data fetching from all sources
async fn fetch_blog_data() -> Vec<BlogArticle> {
    let mut articles = scrape_devto_articles().await;
    articles.extend(scrape_hacker_news().await);
    articles.extend(scrape_reddit().await);
    articles
}

// Routes

#[get("/")]
async fn index() -> Template {
    let articles = fetch_blog_data().await;
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
async fn list_posts() -> Template {
    let articles = fetch_blog_data().await;
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
    let all_articles = fetch_blog_data().await;

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
        .mount("/", routes![index, list_posts, posts_by_category])
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
}
