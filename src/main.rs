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
    source: String,  // Added source field
}

// Article categorization function
fn categorize_article(article: &BlogArticle) -> &'static str {
    match article.source.as_str() {
        "Dev.to" | "Engadget" => "tech",
        "Reddit" | "Medium" | "Mashable" => "lifestyle",
        "The Guardian" | "ProPublica" => "security",
        "Hacker News" => "news",
        _ => "other",
    }
}

// Fetch articles from Dev.to
async fn scrape_devto_articles() -> Vec<BlogArticle> {
    let client = Client::new();
    let url = "https://dev.to/api/articles";
    let mut articles = Vec::new();

    match client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await 
    {
        Ok(response) => {
            if let Ok(posts) = response.json::<Vec<serde_json::Value>>().await {
                for post in posts.iter().take(5) {
                    let article = BlogArticle {
                        id: post["id"].to_string(),
                        title: post["title"].as_str().unwrap_or("No title").to_string(),
                        url: post["url"].as_str().unwrap_or("").to_string(),
                        excerpt: post["description"].as_str().unwrap_or("No excerpt").to_string(),
                        tags: vec!["Dev.to".to_string()],
                        content: post["body_markdown"].as_str().unwrap_or("").to_string(),
                        description: Some(post["description"].as_str().unwrap_or("").to_string()),
                        category: None,
                        source: "Dev.to".to_string(),
                    };
                    articles.push(article);
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
                            let article = BlogArticle {
                                id: story_id.to_string(),
                                title: story["title"].as_str().unwrap_or("No title").to_string(),
                                url: story["url"].as_str().unwrap_or("").to_string(),
                                excerpt: story["text"].as_str().unwrap_or("No description").to_string(),
                                tags: vec!["Hacker News".to_string()],
                                content: story["text"].as_str().unwrap_or("No description").to_string(),
                                description: None,
                                category: None,
                                source: "Hacker News".to_string(),
                            };
                            articles.push(article);
                        }
                    }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        },
        Err(e) => eprintln!("Failed to fetch Hacker News articles: {}", e),
    }

    articles
}

// Fetch articles from Medium
async fn scrape_medium_articles() -> Vec<BlogArticle> {
    let url = "https://medium.com/";
    let client = Client::new();
    let mut articles = Vec::new();

    match client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await 
    {
        Ok(response) => {
            if let Ok(text) = response.text().await {
                let document = scraper::Html::parse_document(&text);
                let article_selector = scraper::Selector::parse("article").unwrap();
                let title_selector = scraper::Selector::parse("h2").unwrap();
                let link_selector = scraper::Selector::parse("a").unwrap();
                let excerpt_selector = scraper::Selector::parse("p").unwrap();

                for article in document.select(&article_selector).take(5) {
                    let title = article.select(&title_selector).next()
                        .map(|e| e.inner_html())
                        .unwrap_or_else(|| "No title".to_string());
                    let link = article.select(&link_selector).next()
                        .and_then(|e| e.value().attr("href"))
                        .unwrap_or("")
                        .to_string();
                    let excerpt = article.select(&excerpt_selector).next()
                        .map(|e| e.inner_html())
                        .unwrap_or_else(|| "No excerpt".to_string());

                    let article = BlogArticle {
                        id: link.split('/').last().unwrap_or("No ID").to_string(),
                        title,
                        url: if link.starts_with("http") { link.clone() } else { format!("https://medium.com{}", link) },
                        excerpt: excerpt.clone(),
                        tags: vec!["Medium".to_string()],
                        content: excerpt,
                        description: None,
                        category: None,
                        source: "Medium".to_string(),
                    };
                    articles.push(article);
                }
            }
        },
        Err(e) => eprintln!("Failed to fetch Medium articles: {}", e),
    }

    articles
}

// Fetch articles from TechCrunch
async fn scrape_techcrunch_articles() -> Vec<BlogArticle> {
    let url = "https://techcrunch.com/";
    let client = Client::new();
    let mut articles = Vec::new();

    match client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await 
    {
        Ok(response) => {
            if let Ok(text) = response.text().await {
                let document = scraper::Html::parse_document(&text);
                let article_selector = scraper::Selector::parse("article").unwrap();
                let title_selector = scraper::Selector::parse("h2").unwrap();
                let link_selector = scraper::Selector::parse("a").unwrap();
                let excerpt_selector = scraper::Selector::parse("div.post-block__content").unwrap();

                for article in document.select(&article_selector).take(5) {
                    let title = article.select(&title_selector).next()
                        .map(|e| e.inner_html())
                        .unwrap_or_else(|| "No title".to_string());
                    let link = article.select(&link_selector).next()
                        .and_then(|e| e.value().attr("href"))
                        .unwrap_or("")
                        .to_string();
                    let excerpt = article.select(&excerpt_selector).next()
                        .map(|e| e.inner_html())
                        .unwrap_or_else(|| "No excerpt".to_string());

                    let article = BlogArticle {
                        id: link.split('/').last().unwrap_or("No ID").to_string(),
                        title,
                        url: link,
                        excerpt: excerpt.clone(),
                        tags: vec!["TechCrunch".to_string()],
                        content: excerpt,
                        description: None,
                        category: None,
                        source: "TechCrunch".to_string(),
                    };
                    articles.push(article);
                }
            }
        },
        Err(e) => eprintln!("Failed to fetch TechCrunch articles: {}", e),
    }

    articles
}

// Fetch articles from The Guardian
async fn scrape_guardian_articles() -> Vec<BlogArticle> {
    let api_key = "YOUR_GUARDIAN_API_KEY"; // Replace with your Guardian API key
    let url = format!(
        "https://content.guardianapis.com/search?section=technology&api-key={}&show-fields=bodyText",
        api_key
    );
    let client = Client::new();
    let mut articles = Vec::new();

    match client.get(&url).send().await {
        Ok(response) => {
            if let Ok(data) = response.json::<serde_json::Value>().await {
                if let Some(results) = data["response"]["results"].as_array() {
                    for result in results.iter().take(5) {
                        let article = BlogArticle {
                            id: result["id"].as_str().unwrap_or("No ID").to_string(),
                            title: result["webTitle"].as_str().unwrap_or("No title").to_string(),
                            url: result["webUrl"].as_str().unwrap_or("").to_string(),
                            excerpt: result["fields"]["bodyText"].as_str().unwrap_or("No content")[..200].to_string() + "...",
                            tags: vec!["The Guardian".to_string()],
                            content: result["fields"]["bodyText"].as_str().unwrap_or("No content").to_string(),
                            description: None,
                            category: None,
                            source: "The Guardian".to_string(),
                        };
                        articles.push(article);
                    }
                }
            }
        },
        Err(e) => eprintln!("Failed to fetch Guardian articles: {}", e),
    }

    articles
}

// Fetch articles from Mashable
async fn scrape_mashable_articles() -> Vec<BlogArticle> {
    let url = "https://mashable.com/tech";
    let client = Client::new();
    let mut articles = Vec::new();

    match client
        .get(url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await 
    {
        Ok(response) => {
            if let Ok(text) = response.text().await {
                let document = scraper::Html::parse_document(&text);
                let article_selector = scraper::Selector::parse("article").unwrap();
                let title_selector = scraper::Selector::parse("h2").unwrap();
                let link_selector = scraper::Selector::parse("a").unwrap();
                let excerpt_selector = scraper::Selector::parse("p.article-description").unwrap();

                for article in document.select(&article_selector).take(5) {
                    let title = article.select(&title_selector).next()
                        .map(|e| e.inner_html())
                        .unwrap_or_else(|| "No title".to_string());
                    let link = article.select(&link_selector).next()
                        .and_then(|e| e.value().attr("href"))
                        .unwrap_or("")
                        .to_string();
                    let excerpt = article.select(&excerpt_selector).next()
                        .map(|e| e.inner_html())
                        .unwrap_or_else(|| "No excerpt".to_string());

                    let article = BlogArticle {
                        id: link.split('/').last().unwrap_or("No ID").to_string(),
                        title,
                        url: if link.starts_with("http") { link.clone() } else { format!("https://mashable.com{}", link) },
                        excerpt: excerpt.clone(),
                        tags: vec!["Mashable".to_string()],
                        content: excerpt,
                        description: None,
                        category: None,
                        source: "Mashable".to_string(),
                    };
                    articles.push(article);
                }
            }
        },
        Err(e) => eprintln!("Failed to fetch Mashable articles: {}", e),
    }

    articles
}

// Combine and categorize data from all sources
async fn fetch_blog_data() -> Vec<BlogArticle> {
    let mut articles = Vec::new();
    
    // Fetch articles from all sources concurrently
    let devto = tokio::spawn(scrape_devto_articles());
    let hacker_news = tokio::spawn(scrape_hacker_news());
    let medium = tokio::spawn(scrape_medium_articles());
    let techcrunch = tokio::spawn(scrape_techcrunch_articles());
    let guardian = tokio::spawn(scrape_guardian_articles());
    let mashable = tokio::spawn(scrape_mashable_articles());

    // Collect and categorize results
    if let Ok(mut devto_articles) = devto.await {
        for article in devto_articles.iter_mut() {
            article.category = Some(categorize_article(&article).to_string());
        }
        articles.extend(devto_articles);
    }
    if let Ok(mut hn_articles) = hacker_news.await {
        for article in hn_articles.iter_mut() {
            article.category = Some(categorize_article(&article).to_string());
        }
        articles.extend(hn_articles);
    }
    if let Ok(mut medium_articles) = medium.await {
        for article in medium_articles.iter_mut() {
            article.category = Some(categorize_article(&article).to_string());
        }
        articles.extend(medium_articles);
    }
    if let Ok(mut techcrunch_articles) = techcrunch.await {
        for article in techcrunch_articles.iter_mut() {
            article.category = Some(categorize_article(&article).to_string());
        }
        articles.extend(techcrunch_articles);
    }
    if let Ok(mut guardian_articles) = guardian.await {
        for article in guardian_articles.iter_mut() {
            article.category = Some(categorize_article(&article).to_string());
        }
        articles.extend(guardian_articles);
    }
    if let Ok(mut mashable_articles) = mashable.await {
        for article in mashable_articles.iter_mut() {
            article.category = Some(categorize_article(&article).to_string());
        }
        articles.extend(mashable_articles);
    }

    articles
}
// Routes remain the same
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
            title: "All Tech News",
            articles: articles,
        },
    )
}

#[get("/category/<tag>")]
async fn posts_by_category(tag: String) -> Template {
    let all_articles = fetch_blog_data().await;

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
            title: format!("Posts from {}", tag),
            articles: filtered_articles,
            category_description: format!("Latest articles from {}", tag),
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