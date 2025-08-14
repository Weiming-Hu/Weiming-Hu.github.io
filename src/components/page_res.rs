use dioxus::prelude::*;
use super::css_preset::*;

#[component]
pub fn Resources() -> Element {
    rsx! {
        div {
            class: format!("{} relative z-10", CSS_CONTENT_CONTAINER),
            
            div {
                class: CSS_CONTENT_CARD,
                
                div {
                    class: "mb-8",
                    h1 {
                        class: CSS_PAGE_TITLE,
                        "Resources"
                    }

                    p {
                        class: "text-gray-600 text-lg leading-relaxed",
                        "I keep this page updated with resources that might help you with your research and learning."
                    }
                }
            }
        }
    }
}
