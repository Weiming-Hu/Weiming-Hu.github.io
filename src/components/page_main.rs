use dioxus::prelude::*;
use super::css_preset::*;
use super::routes::Route;

const SELFIE_IMG: Asset = asset!("/assets/imgs/selfie.jpg");

#[component]
pub fn Home() -> Element {

    rsx! {
        div {
            class: CSS_CONTENT_CONTAINER,
                
                div {
                    class: CSS_CONTENT_CARD,

                    div {
                        class: "mb-8",
                        h1 {
                            class: "text-4xl font-bold text-gray-900 mb-2",
                            "Weiming Hu"
                        }
                        p {
                            class: "text-gray-600 text-lg",
                            "A practical idealist in open source and open science"
                        }
                    }

                    div {
                        class: "flex flex-col lg:flex-row gap-8",
                        
                        div {
                            class: "lg:w-1/3 flex flex-col items-center space-y-6 max-w-[17rem]",
                            img {
                                src: SELFIE_IMG,
                                alt: "Weiming Hu",
                                class: "w-full h-auto rounded-lg shadow-md"
                            }

                            div {
                                class: "mt-1 pt-4 border-t border-gray-200 w-full",
                                div {
                                    class: "text-sm text-gray-600 space-y-1 text-center",
                                    p { "Geography/Geology Building Room 312"}
                                    p { "210 Field St, Athens, GA 30602" }
                                }
                            }
                        }

                        div {
                            class: "flex-1 space-y-4 w-full",
                            
                            p {
                                class: "text-lg text-gray-800",
                                "Salute and welcome!"
                            }

                            p {
                                class: "text-gray-700 leading-relaxed",
                                "I am an "
                                span { class: "font-semibold", "Assistant Professor" }
                                " in the "
                                a { 
                                    href: "https://geography.uga.edu/directory/people/weiming-hu", 
                                    class: "text-red-600 hover:text-red-800 underline",
                                    "Dept. of Geography, Univ. of Georgia"
                                }
                                " and a core faculty member of the "
                                a { 
                                    href: "https://cgr.uga.edu/bio-weiminghu.html", 
                                    class: "text-red-600 hover:text-red-800 underline",
                                    "Center for Geospatial Research"
                                }
                                ". My research interests lie primarily in "
                                span { class: "font-semibold", "machine learning" }
                                " and "
                                span { class: "font-semibold", "big spatiotemporal data analytics" }
                                "."
                            }

                            p {
                                class: "text-gray-700 leading-relaxed",
                                "I am particularly passionate about quantifying and understanding the level of "
                                span { class: "font-semibold", "uncertainty from multi-source data" }
                                ", e.g., remote sensing, model simulations, and ground observations, and from hybrid dynamical-machine-learning models. My goal is to investigate how to build "
                                span { class: "font-style: italic", "accurate, reliable, trustworthy" }
                                ", and "
                                span { class: "font-style: italic", "realistic" }
                                " models with machine learning for "
                                span { class: "font-style: italic", "Environmental and Earth Sciences" }
                                ". My work has been applied to "
                                span { class: "font-semibold", "renewable energy forecasting" }
                                ", "
                                span { class: "font-semibold", "extreme event forecasting" }
                                ", and "
                                span { class: "font-semibold", "water resource management" }
                                "."
                            }
                        }

                    }

                    div {
                        class: "flex items-center mt-6 justify-center",
                        div {
                            class: "bg-red-50 border-l-4 border-red-400 p-4 rounded shadow-sm max-w-2xl",
                            p {
                                class: "text-gray-800 leading-relaxed font-semibold text-center",
                                "I am actively looking for motivated students who are broadly interested in "
                                span { class: "font-bold text-red-700", "geospatial analysis" }
                                ", "
                                span { class: "font-bold text-red-700", "artificial intelligence" }
                                ", "
                                span { class: "font-bold text-red-700", "weather" }
                                ", or "
                                span { class: "font-bold text-red-700", "climate science" }
                                ". Please contact me if you are interested in joining the " 
                                Link { 
                                    to: Route::Director { pagename: "gaim".to_string() },
                                    class: "text-red-700 hover:text-red-900 font-normal",
                                    "Lab for GAIM" 
                                }
                                "."
                            }
                        }
                    }

                    // Social Media Icons
                    div {
                        class: "flex justify-center space-x-6 mt-8 pt-6 border-t border-gray-200",
                        a {
                            href: "https://geography.uga.edu/directory/people/weiming-hu",
                            target: "_blank",
                            class: "text-gray-600 hover:text-red-600 transition-colors text-5xl",
                            title: "University Profile",
                            i { class: "fa-solid fa-address-card" }
                        }
                        a {
                            href: "https://github.com/Weiming-Hu",
                            target: "_blank",
                            class: "text-gray-600 hover:text-red-600 transition-colors text-5xl",
                            title: "GitHub",
                            i { class: "fa-brands fa-github" }
                        }
                        a {
                            href: "https://scholar.google.com/citations?user=2xypOLMAAAAJ&hl",
                            target: "_blank",
                            class: "text-gray-600 hover:text-red-600 transition-colors text-5xl",
                            title: "Google Scholar",
                            i { class: "fa-solid fa-graduation-cap" }
                        }
                    }
                }
            }
    }
}
