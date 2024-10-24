// static/js/scripts.js

document.addEventListener('DOMContentLoaded', function() {
    const body = document.body;
    const themeToggleBtn = document.getElementById('theme-toggle');
    
    console.log('JavaScript is loaded!');

    // Check localStorage for theme preference and apply it
    const theme = localStorage.getItem('theme') || 'light'; // Default to light if no preference is set
    if (theme === 'dark') {
        body.classList.add('bg-dark', 'text-white');
        themeToggleBtn.textContent = 'Toggle Light Mode';
    } else {
        body.classList.add('bg-light', 'text-dark'); // Ensure light mode is set up
        themeToggleBtn.textContent = 'Toggle Dark Mode';
    }

    // Add event listener for theme toggle button
    themeToggleBtn.addEventListener('click', () => {
        const isDarkMode = body.classList.toggle('bg-dark');
        body.classList.toggle('text-white');
        body.classList.toggle('bg-light');
        body.classList.toggle('text-dark');
        
        // Save the theme preference in localStorage
        if (isDarkMode) {
            localStorage.setItem('theme', 'dark');
            themeToggleBtn.textContent = 'Toggle Light Mode';
        } else {
            localStorage.setItem('theme', 'light');
            themeToggleBtn.textContent = 'Toggle Dark Mode';
        }
    });
});
