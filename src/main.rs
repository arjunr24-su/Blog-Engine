#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use rocket::fs::{FileServer, relative};
use pulldown_cmark::{Parser, html};

// Structs for Articles and Hacker News Items
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Article {
    id: i32,
    title: String,
    url: String,
    description: String,
    category: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct HackerNewsItem {
    id: u32,
    title: String,
    url: Option<String>,
}

// Function to fetch blog data from Dev.to
fn fetch_blog_data(api_key: &str) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let url = "https://dev.to/api/articles/me/published";
    let client = Client::new();
    let response = client.get(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()?;

    if response.status().is_success() {
        let articles = response.json::<Vec<Article>>()?;
        Ok(articles)
    } else {
        Err(format!("Error fetching blog data: {}", response.status()).into())
    }
}

// Function to fetch Hacker News data
fn fetch_hacker_news_data() -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let hacker_news_url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    
    let client = Client::new();
    let response = client.get(hacker_news_url).send()?;

    if response.status().is_success() {
        let story_ids: Vec<u32> = response.json()?;
        
        let mut stories = Vec::new();
        for id in story_ids.iter().take(10) {
            let item_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
            let item_response = client.get(&item_url).send()?;

            if item_response.status().is_success() {
                if let Ok(story) = item_response.json::<HackerNewsItem>() {
                    stories.push(Article {
                        id: story.id as i32,
                        title: story.title,
                        url: story.url.unwrap_or_else(|| "https://news.ycombinator.com".to_string()),
                        description: "Hacker News story".to_string(),
                        category: "News".to_string(),
                    });
                }
            }
        }

        Ok(stories)
    } else {
        Err(format!("Error fetching Hacker News data: {}", response.status()).into())
    }
}

// Function to convert Markdown to HTML
fn markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new(markdown);
    html::push_html(&mut html_output, parser);
    html_output
}

#[get("/")]
fn index() -> Template {
    let context = context! {
        title: "Blog Engine",
        message: "This is the home page",
        recent_articles: Vec::<Article>::new(), // Explicitly define the type as Vec<Article>
    };
    Template::render("index", &context)
}

#[get("/post")]
fn render_post() -> Template {
    let api_key = "44h9fR1BfEW98AUVrkNJYVbd"; // Your Dev.to API Key
    let mut devto_articles = fetch_blog_data(&api_key).unwrap_or_else(|_| vec![]);
    let hacker_news_articles = fetch_hacker_news_data().unwrap_or_else(|_| vec![]);

    // Ensure devto_articles is mutable
    for article in &mut devto_articles {
        article.description = markdown_to_html(&article.description);
    }

    let context = context! {
        title: "External Blog Posts",
        devto_articles,
        hacker_news_articles,
    };

    Template::render("blog", &context)
}

#[get("/category/<category_name>")]
fn category_page(category_name: &str) -> Template {
    let context = context! {
        category: category_name,
        posts: vec!["Post 1", "Post 2"],  // Replace with actual filtered posts
    };
    Template::render("category", &context)
}

#[get("/posts")]
fn list_posts() -> Template {
    let api_key = "44h9fR1BfEW98AUVrkNJYVbd"; // Your Dev.to API Key
    let devto_articles = fetch_blog_data(&api_key).unwrap_or_else(|_| vec![]);
    let hacker_news_articles = fetch_hacker_news_data().unwrap_or_else(|_| vec![]);
    
    // Make sure devto_articles is mutable
    let mut all_articles = devto_articles.clone(); // Now this is valid
    all_articles.extend(hacker_news_articles);

    let context = context! {
        title: "All Blog Posts",
        articles: all_articles,
    };

    Template::render("posts", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, render_post, category_page, list_posts])
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
}
