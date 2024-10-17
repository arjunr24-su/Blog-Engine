#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use rocket::fs::{FileServer, relative};

#[derive(Deserialize, Serialize, Debug)]
struct Article {
    id: i32,
    title: String,
    url: String,
    description: String,
}

#[get("/")]
fn index() -> Template {
    let context = context! {
        title: "Blog Engine",
        message: "This is the home page"
    };
    Template::render("index", &context)
}

#[get("/post")]
fn render_post() -> Template {
    // Fetch data from external source using reqwest
    let articles = fetch_blog_data();

    // Pass the articles to the template
    let context = context! {
        title: "External Blog Posts",
        articles: articles,
    };

    Template::render("blog", &context)
}

// Function to fetch blog data using reqwest
fn fetch_blog_data() -> Vec<Article> {
    let api_key = "44h9fR1BfEW98AUVrkNJYVbd"; // Your API key
    let url = "https://dev.to/api/articles/me/published"; // Example endpoint

    let client = Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .expect("Failed to send request");

    if response.status().is_success() {
        response.json::<Vec<Article>>().unwrap_or_else(|_| {
            println!("Failed to parse JSON, returning empty vector");
            vec![] // Return empty vector on parse failure
        })
    } else {
        println!("Error fetching blog data: {}", response.status());
        vec![] // Return empty vector on request failure
    }
}

#[get("/category/<category_name>")]
fn category_page(category_name: &str) -> Template {
    let context = context! {
        category: category_name,
        posts: vec!["Post 1", "Post 2"],  // Example posts
    };
    Template::render("category", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, render_post, category_page])
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static"))) // Ensure this is only included once
}
