use dioxus::prelude::*;
use super::css_preset::*;

const CSS_HLINE: &str = "border-b border-gray-200 pb-6";
const CSS_SECTION: &str = "text-2xl font-bold text-red-900 mb-4";
const GITHUB_STAT_URL: &str = "https://github-readme-stats.vercel.app/api?username=Weiming-Hu&theme=default&show_icons=true&hide_rank=true";
const CSS_LANGUAGE_TAG: &str = "inline-flex items-center px-2 py-1 text-xs font-medium bg-red-100 text-gray-500 rounded-full";

#[component]
pub fn Code() -> Element {
    rsx! {
        div {
            class: CSS_CONTENT_CONTAINER,
            
            div {
                class: CSS_CONTENT_CARD,
                
                // Header
                div {
                    class: "mb-8 flex flex-col lg:flex-row lg:items-start lg:justify-between gap-6",
                    
                    div {
                        class: "lg:w-1/2",
                        h1 {
                            class: CSS_PAGE_TITLE,
                            "Code"
                        }
                        p {
                            class: "text-gray-600 text-lg",
                            "Open source contributions and research software"
                        }
                        
                        // Programming Languages
                        div {
                            class: "mt-4",
                            p {
                                class: "text-sm text-gray-600 mb-2",
                                a { 
                                    href: "https://github.com/Weiming-Hu",
                                    target: "_blank",
                                    class: "mb-2 hover:text-red-700 transition-colors",
                                    "Full profile on GitHub."
                                }
                                " Primary languages:"
                            }
                            div {
                                class: "flex flex-wrap gap-2",
                                span {
                                    class: CSS_LANGUAGE_TAG,
                                    "Python"
                                }
                                span {
                                    class: CSS_LANGUAGE_TAG,
                                    "R"
                                }
                                span {
                                    class: CSS_LANGUAGE_TAG,
                                    "C/C++"
                                }
                                span {
                                    class: CSS_LANGUAGE_TAG,
                                    "Rust"
                                }
                            }
                        }
                    }
                    
                    div {
                        class: "lg:w-1/2 flex justify-center lg:justify-end",
                        img {
                            src: GITHUB_STAT_URL,
                            alt: "GitHub Stats",
                            style: "height: 200px; border: none;",
                        }
                    }
                }

                // GitHub Stats Section
                div {
                    class: "space-y-6",
                    
                    div {
                        class: "{CSS_HLINE}",
                        h2 {
                            class: "{CSS_SECTION}",
                            "Selected Repositories"
                        }
                        
                        RepositoryCard {
                            name: "AnalogsEnsemble",
                            description: "The C++ and R packages for parallel ensemble forecasts using Analog Ensemble",
                            language: "C++",
                            url: "https://github.com/Weiming-Hu/AnalogsEnsemble",
                            topics: vec!["r-package".to_string(), "forecasting".to_string(), "weather".to_string(), "uncertainty".to_string()]
                        }
                        
                        RepositoryCard {
                            name: "RAnEnExtra",
                            description: "This is a R package that contains helpful functions for the RAnEn package.",
                            language: "R",
                            url: "https://github.com/Weiming-Hu/RAnEnExtra/",
                            topics: vec!["ensemble-forecasting".to_string(), "verification".to_string(), "analysis".to_string()]
                        }
                        
                        RepositoryCard {
                            name: "DeepAnalogs",
                            description: "Deep learning approach to analog ensemble forecasting using neural networks for spatiotemporal pattern recognition",
                            language: "Python",
                            url: "https://github.com/Weiming-Hu/DeepAnalogs",
                            topics: vec!["deep-learning".to_string(), "pytorch".to_string(), "forecasting".to_string()]
                        }
                        
                        RepositoryCard {
                            name: "PyPIOMAS",
                            description: "Module for downloading and converting PIOMAS data",
                            language: "Python",
                            url: "https://github.com/Weiming-Hu/PyPIOMAS",
                            topics: vec!["PIOMAS".to_string(), "arctic".to_string()]
                        }
                    }
                }
            }
        }
    }
}
#[component]
fn RepositoryCard(
    name: String,
    description: String,
    language: String,
    url: String,
    topics: Vec<String>,
) -> Element {
    rsx! {
        a {
            href: "{url}",
            target: "_blank",
            class: "block mb-4 p-6 border border-gray-200 rounded-lg bg-white hover:bg-gray-50 hover:shadow-lg hover:border-red-300 transition-all duration-200 cursor-pointer transform hover:-translate-y-1",
            
            div {
                class: "flex items-start justify-between mb-3",
                h3 {
                    class: "text-xl font-semibold text-gray-900 hover:text-red-700 transition-colors",
                    "{name}"
                }
            }
            
            p {
                class: "text-gray-700 mb-4 leading-relaxed",
                "{description}"
            }
            
            div {
                class: "flex items-center justify-between",
                div {
                    class: "flex items-center",
                    span {
                        class: "w-3 h-3 rounded-full mr-2 bg-gray-600",
                    }
                    span {
                        class: "text-sm font-medium text-gray-600",
                        "{language}"
                    }
                }
                
                div {
                    class: "flex flex-wrap gap-1",
                    for topic in topics.iter().take(3) {
                        span {
                            class: CSS_LANGUAGE_TAG,
                            "{topic}"
                        }
                    }
                }
            }
        }
    }
}
