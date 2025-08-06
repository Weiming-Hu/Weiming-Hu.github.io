use dioxus::prelude::*;
use super::css_preset::*;

#[component]
pub fn Info() -> Element {
    rsx! {
        div {
            class: format!("{} relative z-10", CSS_CONTENT_CONTAINER),
            
            div {
                class: CSS_CONTENT_CARD,
                
                div {
                    class: "mb-8",
                    h1 {
                        class: CSS_PAGE_TITLE,
                        "Information"
                    }

                    p {
                        class: "text-gray-600 italic",
                        "Website version: "
                        span { 
                            class: "font-mono font-semibold",
                            "{env!(\"CARGO_PKG_VERSION\")}"
                        }
                    }

                }
            }
        }
    }
}
