# Blog Engine Project Report

### Project Overview
You are developing a blog engine using Rust with the Rocket framework. The project aims to create a fully functional online platform to render and display blog content, featuring content aggregation, Markdown parsing, post organization, and dynamic routing.

---

### Current Status

#### Features Implemented

1. **Basic Structure:**
   - The project is structured into directories for templates and static files (CSS/JavaScript).
   - The main file is `src/main.rs`, where the Rocket server is set up.

2. **Routing:**
   - **Home Page:** Implemented at the root path (`/`), displaying a welcome message.
   - **Post Rendering:** Implemented a route (`/post`) to fetch and display blog articles from an external API (Dev.to).
   - **Category Pages:** Implemented a route (`/category/<category_name>`) to display posts by category, with example posts.

3. **Template Rendering:**
   - Integrated Tera templates for rendering HTML pages.
   - The `index.html.tera` file has been set up to include a basic layout and navigation.

4. **JavaScript Integration:**
   - A JavaScript file (`scripts.js`) has been created to handle dark/light mode toggling.
   - Utilized `localStorage` to remember the user's theme preference.

5. **External API Integration:**
   - Successfully fetching blog articles from Dev.to using the `reqwest` crate.
   - Added functionality to fetch top stories from Hacker News and combined articles from both sources into a single list for display.

6. **Markdown Rendering:**
   - Implemented Markdown parsing and rendering for blog post descriptions using `pulldown-cmark`.

7. **Static Files:**
   - Set up a static file server to serve CSS and JavaScript files.
   - Bootstrap has been integrated for styling.

---

### Technologies Used

- **Rust:** The programming language for the backend.
- **Rocket:** The web framework for building the server.
- **Tera:** The templating engine for HTML.
- **Reqwest:** The HTTP client for fetching external data.
- **Bootstrap:** For responsive CSS styling.

---

### Requirements Matching

| Requirement Description                                       | Current Implementation                                             | Matches (Yes/No) |
|-------------------------------------------------------------|------------------------------------------------------------------|------------------|
| Develop a blog engine to render data as Markdown content    | Rendering blog content fetched from an external API              | Yes              |
| Aggregate blog posts from multiple sources                   | Implemented fetching posts from Dev.to API                       | Yes              |
| Dynamic routing for different pages                          | Routes for home, posts, and categories are implemented           | Yes              |
| Basic templating for managing dynamic content                | Tera templates are being used for rendering HTML                 | Yes              |
| Styling using CSS                                           | Integrated Bootstrap for styling                                  | Yes              |
| JavaScript for front-end interactivity                      | Implemented theme toggle functionality using JavaScript           | Yes              |
| Markdown parsing for content                                 | Implemented for blog post descriptions                           | Yes              |
| Organizing posts by categories                               | Basic category route implemented but without real content        | Partially        |
| Fetching and displaying external blog content                | Successfully fetching data from external API                     | Yes              |

---

### Challenges Faced
- Encountered issues with fetching data from external APIs.
- Handled error cases where the data retrieval might fail, reverting to fake data as a fallback.
- Needed to enhance error handling for API requests and data parsing.

---

### Pending Tasks

1. **Markdown Parsing:** Implement functionality to parse fetched content into Markdown and render it correctly (if not yet fully implemented).
   
2. **Dynamic Content for Categories:** Populate category pages with actual blog posts instead of placeholder data.

3. **UI Improvements:** Refine the user interface for better aesthetics and usability.

4. **Testing:** Write unit tests and integration tests to ensure the reliability of features.

5. **Documentation:** Create comprehensive documentation for the code and usage of the blog engine.

---

### Conclusion
The project is progressing well, with a significant portion of the foundational features implemented. Most of the key requirements are met, while a few remain in progress. Continued focus on the remaining tasks will be essential to complete the project successfully.

--- 

