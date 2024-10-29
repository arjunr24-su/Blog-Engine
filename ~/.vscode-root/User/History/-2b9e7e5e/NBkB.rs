#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};
use reqwest::Client;

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

#[derive(Debug, Serialize)]
struct Pagination {
    current_page: u32,
    has_previous: bool,
    has_next: bool,
    previous_page_url: Option<String>,
    next_page_url: Option<String>,
    pages: Vec<Page>,
}

#[derive(Debug, Serialize)]
struct Page {
    number: u32,
    url: String,
}

async fn scrape_devto_articles() -> Vec<BlogArticle> {
    let url = "https://dev.to";
    let client = Client::new();
    let mut articles = Vec::new();

    if let Ok(response) = client.get(url).send().await {
        if let Ok(text) = response.text().await {
            let document = scraper::Html::parse_document(&text);
            let post_selector = scraper::Selector::parse("div.crayons-story").unwrap();
            let title_selector = scraper::Selector::parse("h2.crayons-story__title").unwrap();
            let link_selector = scraper::Selector::parse("a.crayons-story__hidden-navigation").unwrap();
            let excerpt_selector = scraper::Selector::parse("p.crayons-story__description").unwrap();

            for post in document.select(&post_selector).take(5) {
                let title = post.select(&title_selector).next().map(|e| e.inner_html()).unwrap_or("No title".to_string());
                let link = post.select(&link_selector).next().and_then(|e| e.value().attr("href")).unwrap_or("").to_string();
                let excerpt = post.select(&excerpt_selector).next().map(|e| e.inner_html()).unwrap_or("No excerpt".to_string());

                articles.push(BlogArticle {
                    id: link.split('/').last().unwrap_or("No ID").to_string(),
                    title,
                    url: format!("{}{}", url, link),
                    excerpt: excerpt.clone(),
                    tags: vec!["tech".to_string()],
                    content: excerpt,
                    description: None,
                    category: Some("Tech".to_string()),
                });
            }
        }
    }
    articles
}

async fn scrape_wired_articles() -> Vec<BlogArticle> {
    let url = "https://www.wired.com";
    let client = Client::new();
    let mut articles = Vec::new();

    if let Ok(response) = client.get(url).send().await {
        if let Ok(text) = response.text().await {
            let document = scraper::Html::parse_document(&text);
            let article_selector = scraper::Selector::parse("article").unwrap();
            let title_selector = scraper::Selector::parse("h2").unwrap();
            let link_selector = scraper::Selector::parse("a").unwrap();
            let excerpt_selector = scraper::Selector::parse("p").unwrap();

            for article in document.select(&article_selector).take(5) {
                let title = article.select(&title_selector).next().map(|e| e.inner_html()).unwrap_or("No title".to_string());
                let link = article.select(&link_selector).next().and_then(|e| e.value().attr("href")).unwrap_or("").to_string();
                let excerpt = article.select(&excerpt_selector).next().map(|e| e.inner_html()).unwrap_or("No excerpt".to_string());

                articles.push(BlogArticle {
                    id: link.split('/').last().unwrap_or("No ID").to_string(),
                    title,
                    url: format!("{}{}", url, link),
                    excerpt: excerpt.clone(),
                    tags: vec!["tech".to_string(), "security".to_string()],
                    content: excerpt,
                    description: None,
                    category: Some("Tech & Security".to_string()),
                });
            }
        }
    }
    articles
}

async fn scrape_reddit() -> Vec<BlogArticle> {
    let mut articles = Vec::new();
    let client = Client::new();
    let url = "https://www.reddit.com/r/popular.json";

    if let Ok(response) = client.get(url).header("User-Agent", "rust-app").send().await {
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
                        tags: vec!["lifestyle".to_string(), "news".to_string()],
                        content: excerpt,
                        description: None,
                        category: Some("Lifestyle & News".to_string()),
                    });
                }
            }
        }
    }
    articles
}

async fn fetch_blog_data() -> Vec<BlogArticle> {
    let mut articles = scrape_devto_articles().await;
    articles.extend(scrape_wired_articles().await);
    articles.extend(scrape_reddit().await);
    articles
}

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

#[get("/category/<tag>?<page>")]
async fn posts_by_category(tag: String, page: Option<u32>) -> Template {
    let all_articles = fetch_blog_data().await;
    let articles_per_page = 10;
    let page = page.unwrap_or(1);
    
    let filtered_articles: Vec<BlogArticle> = all_articles
        .into_iter()
        .filter(|article| {
            article
                .tags
                .iter()
                .any(|t| t.to_lowercase() == tag.to_lowercase())
        })
        .collect();

    let paginated_articles = filtered_articles
        .chunks(articles_per_page)
        .nth((page - 1) as usize)
        .unwrap_or(&[])
        .to_vec();

    let pagination = Pagination {
        current_page: page,
        has_previous: page > 1,
        has_next: (page as usize) * articles_per_page < filtered_articles.len(),
        previous_page_url: if page > 1 { Some(format!("/category/{}?page={}", tag, page - 1)) } else { None },
        next_page_url: if (page as usize) * articles_per_page < filtered_articles.len() { 
            Some(format!("/category/{}?page={}", tag, page + 1)) 
        } else { 
            None 
        },
        pages: (1..=((filtered_articles.len() as f32 / articles_per_page as f32).ceil() as u32))
            .map(|p| Page { number: p, url: format!("/category/{}?page={}", tag, p) })
            .collect(),
    };

    Template::render(
        "category",
        context! {
            title: format!("Posts in Category: {}", tag),
            articles: paginated_articles,
            category_description: format!("All articles in the {} category", tag),
            pagination,
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
