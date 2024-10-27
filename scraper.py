import requests
from bs4 import BeautifulSoup
import json
import time

def scrape_devto_articles():
    url = "https://dev.to"
    articles = []

    try:
        response = requests.get(url)
        response.raise_for_status()  # Raise an error for bad responses

        soup = BeautifulSoup(response.text, 'html.parser')
        posts = soup.find_all('div', class_='crayons-story')[:5]  # Limit to top 5 posts

        for post in posts:
            title_elem = post.find('h2', class_='crayons-story__title')
            link_elem = post.find('a', class_='crayons-story__hidden-navigation')
            excerpt_elem = post.find('p', class_='crayons-story__description')

            title = title_elem.text.strip() if title_elem else "No title"
            link = link_elem['href'] if link_elem else ""
            excerpt = excerpt_elem.text.strip() if excerpt_elem else ""

            articles.append({
                "id": link.split('/')[-1] if link else "No ID",
                "title": title,
                "url": url + link if link else url,
                "excerpt": excerpt,
                "tags": ["Dev.to"],
                "content": excerpt  # Consider fetching full content if needed
            })
    except requests.RequestException as e:
        print(f"Failed to fetch Dev.to articles: {e}")

    return articles

def scrape_hacker_news():
    articles = []
    url = "https://hacker-news.firebaseio.com/v0/topstories.json"

    try:
        response = requests.get(url)
        response.raise_for_status()  # Raise an error for bad responses

        top_ids = response.json()[:5]  # Limit to top 5 stories
        for story_id in top_ids:
            story_url = f"https://hacker-news.firebaseio.com/v0/item/{story_id}.json"
            story_response = requests.get(story_url)

            if story_response.status_code == 200:
                story = story_response.json()
                articles.append({
                    "id": str(story_id),
                    "title": story.get("title", "No title"),
                    "url": story.get("url", ""),
                    "excerpt": story.get("text", "No description"),
                    "tags": ["Hacker News"],
                    "content": story.get("text", "")
                })
            time.sleep(1)  # Sleep to avoid hitting the server too hard
    except requests.RequestException as e:
        print(f"Failed to fetch Hacker News articles: {e}")

    return articles

if __name__ == "__main__":
    devto_articles = scrape_devto_articles()
    time.sleep(1)  # Sleep between scraping different sites
    hacker_news_articles = scrape_hacker_news()
    all_articles = devto_articles + hacker_news_articles

    # Print as JSON to pass back to Rust
    print(json.dumps(all_articles, ensure_ascii=False))

