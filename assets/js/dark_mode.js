document.addEventListener('DOMContentLoaded', function() {
    const mode_toggle = document.getElementById("light-toggle");

    mode_toggle.addEventListener("click", function() {
        toggleTheme(localStorage.getItem("theme"));
        if (window.location.pathname == "/") {
            let theme = localStorage.getItem("theme");
            if (theme == "dark") {
                document.documentElement.style.backgroundImage = 'url(/assets/img/background_dark.jpg)';
            } else {
                document.documentElement.style.backgroundImage = 'url(/assets/img/background_light.jpg)';
            }
        }
    });
});

