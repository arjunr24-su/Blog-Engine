// static/js/scripts.js

document.addEventListener('DOMContentLoaded', function() {
    const body = document.body;
    const themeToggleBtn = document.getElementById('theme-toggle');
    
    console.log('JavaScript is loaded!');

    // Check localStorage for theme preference
    if (localStorage.getItem('theme') === 'dark') {
        body.classList.add('bg-dark', 'text-white');
        body.classList.remove('bg-light', 'text-dark');
        themeToggleBtn.textContent = 'Toggle Light Mode';
    }

    // Add event listener for theme toggle button
    themeToggleBtn.addEventListener('click', () => {
        body.classList.toggle('bg-dark');
        body.classList.toggle('text-white');
        body.classList.toggle('bg-light');
        body.classList.toggle('text-dark');
        
        // Save the theme preference in localStorage
        if (body.classList.contains('bg-dark')) {
            localStorage.setItem('theme', 'dark');
            themeToggleBtn.textContent = 'Toggle Light Mode';
        } else {
            localStorage.setItem('theme', 'light');
            themeToggleBtn.textContent = 'Toggle Dark Mode';
        }
    });
});
