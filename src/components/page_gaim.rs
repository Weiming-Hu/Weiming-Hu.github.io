use dioxus::prelude::*;
use super::css_preset::*;

const UGA_LOGO: Asset = asset!("/assets/imgs/uga_logo.png");
const PPL_IMAGE_HU: Asset = asset!("/assets/ppl/hu.png");
const CSS_MEMBER_TEXT: &str = "text-gray-700 leading-relaxed";

#[component]
fn TeamMember(
    first_name: String,
    last_name: String,
    role: String,
    image_path: String,
    description: Element,
) -> Element {
    rsx! {
        div {
            class: "bg-white p-4 rounded-lg shadow-md border border-gray-200 flex flex-col md:flex-row gap-8",
            // Left column: Image and name
            div {
                class: "flex flex-col items-center justify-center md:items-center md:justify-center md:w-1/4 md:h-full md:self-center",
                img {
                    src: "{image_path}",
                    alt: "{first_name} {last_name}",
                    class: "aspect-square w-full h-auto max-w-[250px] max-h-[250px] rounded-full object-cover shadow-md m-4"
                }
                h3 {
                    class: "text-xl font-semibold text-gray-800 text-center md:text-left",
                    "{first_name} {last_name}"
                }
                p {
                    class: "text-red-600 font-medium text-center md:text-left",
                    "{role}"
                }
            }
            // Right column: Description
            div {
                class: "flex-1 md:w-3/4 md:h-full md:self-center space-y-3 text-lg",
                {description}
            }
        }
    }
}

#[component]
pub fn Lab() -> Element {
    rsx! {
        div {
            class: format!("{} relative z-10", CSS_CONTENT_CONTAINER),
            
            div {
                class: CSS_CONTENT_CARD,
                
                div {
                    class: "mb-8 space-y-4 mb-6 border-b-2 border-red-600 flex flex-col sm:flex-row sm:justify-between sm:items-center",
                    div {
                        class: "flex-1",
                        h1 {
                            class: CSS_PAGE_TITLE,
                            "Lab for "
                            span {
                                style: "background: linear-gradient(90deg, #000000, #ba0c2f, #e4002b, #000000, #e4002b, #ffd200); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text; color: transparent;",
                                "Geoinformatics and AI Modeling"
                            }
                            " (GAIM)"
                        }
                    }
                    div {
                        class: "flex justify-center sm:justify-end sm:flex-shrink-0 sm:ml-8 mt-4 sm:mt-0",
                        img {
                            src: UGA_LOGO,
                            alt: "University of Georgia Logo",
                            class: "h-24 w-auto object-contain"
                        }
                    }
                }

                // About Us Section
                div {
                    class: "mb-12 space-y-6",
                    h2 {
                        class: "text-3xl font-bold text-gray-800",
                        "About Us"
                    }
                    
                    p {
                        class: "text-gray-600 text-lg leading-relaxed",
                        "We focus on advancing geospatial analytics and predictive
                        modeling by combining Artificial Intelligence (AI) with Geoinformatics. "
                        "AI means learning from data to uncover patterns, make predictions, and quantify uncertainty
                        in ways that adapt and scale. Geoinformatics grounds these capabilities in space and time,
                        connecting models to the physical world through remote sensing and GIS."
                    }

                    p {
                        class: "text-gray-600 text-lg leading-relaxed",
                        "We develop scalable, uncertainty-aware methods to study and forecast phenomena such as
                        extreme events and renewable energy (wind and solar photovoltaic) production. "
                        i {
                            class: "hover:text-red-700 transition-colors",
                            "To integrate Geoinformatics and AI is not just to interpret the Earth as it is,
                            but to anticipate how it will evolve in the future, both near and far."
                        }
                    }

                    div {
                        class: "bg-yellow-50 border-l-4 border-yellow-400 p-4 rounded-lg flex justify-center max-w-2xl items-center gap-3 mx-auto",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-6 w-6 text-yellow-500 flex-shrink-0",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M13 16h-1v-4h-1m1-4h.01M12 20a8 8 0 100-16 8 8 0 000 16z"
                            }
                        }
                        span {
                            class: "text-yellow-900 font-medium text-lg",
                            "Students and early-career scholars: check out "
                            a {
                                href: "/res",
                                target: "_blank",
                                class: CSS_LINK_TEXT,
                                "our resource page"
                            }
                            "!"
                        }
                    }
                }

                // Research Themes Section
                div {
                    class: "mb-12 space-y-6",
                    h2 {
                        class: "text-3xl font-bold text-gray-800 mb-6 pb-2",
                        "Research Themes"
                    }

                    div {
                        class: "grid gap-6 md:grid-cols-1 lg:grid-cols-3",
                        
                        // Theme 1: Extreme Events
                        a {
                            href: "https://doi.org/10.1175/MWR-D-22-0268.1",
                            target: "_blank",
                            class: "block mb-4 p-4 border-l-4 border-red-400 rounded-r-lg bg-white hover:bg-gray-50 hover:shadow-lg hover:border-red-500 transition-all duration-200 cursor-pointer transform hover:-translate-y-1",
                            div {
                                class: "flex items-center gap-4",
                                div {
                                    class: "flex-shrink-0 w-12 h-12 bg-gray-100 rounded-lg flex items-center justify-center",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "w-6 h-6 text-gray-600",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            d: "M2.25 15a4.5 4.5 0 004.5 4.5H18a3.75 3.75 0 001.332-7.257 3 3 0 00-3.758-3.848 5.25 5.25 0 00-10.233 2.33A4.502 4.502 0 002.25 15z"
                                        }
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            d: "M11 19l-2 3m4-3l-2 3m4-3l-2 3"
                                        }
                                    }
                                }
                                div {
                                    class: "flex-1",
                                    span {
                                        class: "text-lg font-medium text-gray-800",
                                        "Resilience against extreme events with better forecasts: heatwaves and precipitation"
                                    }
                                }
                            }
                        }

                        // Theme 2: Energy Market
                        a {
                            href: "https://www.sciencedirect.com/science/article/pii/S2352340922000361",
                            target: "_blank",
                            class: "block mb-4 p-4 border-l-4 border-red-400 rounded-r-lg bg-white hover:bg-gray-50 hover:shadow-lg hover:border-red-500 transition-all duration-200 cursor-pointer transform hover:-translate-y-1",
                            div {
                                class: "flex items-center gap-4",
                                div {
                                    class: "flex-shrink-0 w-12 h-12 bg-gray-100 rounded-lg flex items-center justify-center",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "w-6 h-6 text-gray-600",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            d: "M13 10V3L4 14h7v7l9-11h-7z"
                                        }
                                    }
                                }
                                div {
                                    class: "flex-1",
                                    span {
                                        class: "text-lg font-medium text-gray-800",
                                        "AI for energy market forecasting and renewable energy production optimization"
                                    }
                                }
                            }
                        }

                        // Theme 3: Arctic Sustainability
                        a {
                            href: "https://doi.org/10.1080/19475683.2024.2380678",
                            target: "_blank",
                            class: "block mb-4 p-4 border-l-4 border-red-400 rounded-r-lg bg-white hover:bg-gray-50 hover:shadow-lg hover:border-red-500 transition-all duration-200 cursor-pointer transform hover:-translate-y-1",
                            div {
                                class: "flex items-center gap-4",
                                div {
                                    class: "flex-shrink-0 w-12 h-12 bg-gray-100 rounded-lg flex items-center justify-center",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "w-6 h-6 text-gray-600",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            d: "M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                        }
                                    }
                                }
                                div {
                                    class: "flex-1",
                                    span {
                                        class: "text-lg font-medium text-gray-800",
                                        "Sustainable accessibility in the Arctic with geospatial modeling"
                                    }
                                }
                            }
                        }
                    }
                }

                // People Section
                // div {
                //     class: "mb-12 space-y-6",
                //     h2 {
                //         class: "text-3xl font-bold text-gray-800 mb-6 pb-2",
                //         "People"
                //     }

                //     div {
                //         class: "space-y-8",
                        
                //         TeamMember {
                //             first_name: "Weiming".to_string(),
                //             last_name: "Hu".to_string(),
                //             role: "Principal Investigator".to_string(),
                //             image_path: PPL_IMAGE_HU,
                //             description: rsx!{ 
                //                 p {  
                //                     class: CSS_MEMBER_TEXT,
                //                     "My research spans Geographic Information Science, Machine Learning, and environmental forecasting, "
                //                     "with expertise in extreme and rare event prediction (heatwaves, precipitation, flooding), renewable energy "
                //                     "forecasting (solar and wind), and uncertainty quantification in spatio-temporal big data. "
                //                     "I have developed "
                //                     a {
                //                         href: "https://cw3e.ucsd.edu/ml_forecasts/",
                //                         target: "_blank",
                //                         class: CSS_LINK_TEXT,
                //                         "Deep Learning"
                //                     }
                //                     " and "
                //                     a {
                //                         href: "http://weiming.uga.edu/AnalogsEnsemble/2018/12/14/AnEn-explained.html",
                //                         target: "_blank",
                //                         class: CSS_LINK_TEXT,
                //                         "Analog Ensemble"
                //                     }
                //                     " methods to improve predictions over different time scales including weather and sub-seasonal-to-seasonal. "
                //                 },

                //                 p { 
                //                     class: CSS_MEMBER_TEXT,
                //                     "Prior to joining UGA, I conducted research at the "
                //                     a {
                //                         href: "https://cw3e.ucsd.edu/",
                //                         target: "_blank",
                //                         class: CSS_LINK_TEXT,
                //                         "Center of Western Water and Weather Extremes"
                //                     }
                //                     " at Scripps, UC San Diego, and then held a faculty position at "
                //                     a {
                //                         href: "https://www.jmu.edu/cise/index.shtml",
                //                         target: "_blank",
                //                         class: CSS_LINK_TEXT,
                //                         "James Madison University"
                //                     }
                //                     ". "
                //                     "My work bridges computational problem-solving with environmental applications, "
                //                     "often in collaboration with interdisciplinary teams and operational forecasting agencies."
                //                  }
                //             }
                //         }
                //     }
                // }
            }
        }
    }
}
