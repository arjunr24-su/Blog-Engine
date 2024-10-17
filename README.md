### Blog Engine Project Progress Report

**Current Date:** October 17, 2024  
**Deadline:** October 27, 2024  

#### Overview
The project involves developing a blog engine using Rust, leveraging frameworks such as Rocket for backend development and Tera for templating. The aim is to create a fully functional online application that aggregates blog posts and presents them in a cohesive style.

#### Progress Made
1. **Basic Structure Established:**
   - Created the foundational Rust project using Cargo.
   - Implemented routes for the home page and post rendering.

2. **Data Fetching:**
   - Successfully integrated the `reqwest` library to fetch data from the Dev.to API.
   - Defined a data structure (`Article`) to deserialize the JSON response from the API.

3. **Templating:**
   - Set up Tera templating engine to render HTML templates.
   - Developed initial templates for the index and post pages.

4. **Static File Serving:**
   - Configured the project to serve static files (like CSS and JavaScript) from a specified directory.

#### Errors and Challenges Faced
1. **Compilation Errors:**
   - Encountered issues related to missing dependencies and features in `rocket_dyn_templates`. Resolved by enabling necessary features in `Cargo.toml`.

2. **Serialization Issues:**
   - Faced errors indicating that the `Article` struct needed to implement the `Serialize` trait. This was fixed by deriving `Serialize` in the struct definition.

3. **Template Parsing Errors:**
   - Received an error during template initialization due to a syntax issue in the Tera template. This required debugging the template to ensure proper syntax and closing tags.

4. **Rocket Fairing Failure:**
   - Experienced a failure to launch Rocket due to issues with the templating engine initialization, requiring adjustments to the template file.

#### Current Status
- The project has a working structure with routes and fetching capabilities in place. 
- The template rendering functionality has been partially implemented but requires debugging to fully operationalize.
- Further development is needed on additional features, including category handling and UI enhancements.

#### Next Steps
- **Debug Tera Templates:** Fix syntax issues in the Tera template files to ensure they parse correctly.
- **Enhance Functionality:** Implement additional features such as category pages and dynamic content rendering.
- **Testing and Validation:** Conduct thorough testing to ensure data fetching and template rendering work as intended.
- **UI Improvements:** Refine the user interface for better aesthetics and usability.

#### Timeline
- **October 17 - 22:** Focus on debugging templates and enhancing core features.
- **October 23 - 25:** Testing and validation of functionalities.
- **October 26:** Final refinements and preparation for submission.
- **October 27:** Project deadline.

This report summarizes the progress and challenges faced in developing the blog engine. The remaining time will be utilized effectively to meet the project deadline.
