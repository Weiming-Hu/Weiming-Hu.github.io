mod components;

use dioxus::prelude::*;
use components::Route;

const FAVICON: Asset = asset!("/assets/icons/favicon.svg");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind_output.css");
const FA_CSS: &str = "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(move || {
        document::eval(
            r#"
            // Handle hash-based routing
            function handleHashNavigation() {
                const hash = window.location.hash;
                // console.log('Hash detected:', hash);
                
                if (hash.startsWith('#/')) {
                    const path = hash.substring(1); // Remove the #
                    // console.log('Navigating to path:', path);
                    
                    // Use history.replaceState to avoid adding to browser history
                    window.history.replaceState(null, '', path);
                    
                    // Create a custom event to trigger router update
                    const event = new CustomEvent('hashroute', { detail: { path } });
                    window.dispatchEvent(event);
                    
                    // Also trigger popstate for good measure
                    window.dispatchEvent(new PopStateEvent('popstate', { state: null }));
                }
            }
            
            // Check hash on page load
            if (window.location.hash) {
                // Small delay to ensure router is ready
                setTimeout(handleHashNavigation, 100);
            }
            
            // Listen for hash changes
            window.addEventListener('hashchange', handleHashNavigation);

            document.body.classList.add('loaded');
            "#
        );
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: FA_CSS }
        document::Link { rel: "icon", href: FAVICON }
        
        Router::<Route> {}
    }
}
