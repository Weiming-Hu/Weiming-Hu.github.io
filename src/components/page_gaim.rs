use dioxus::prelude::*;
use super::css_preset::*;

#[component]
pub fn Lab() -> Element {
    rsx! {
        div {
            class: format!("{} relative z-10", CSS_CONTENT_CONTAINER),
            
            div {
                class: CSS_CONTENT_CARD,
                
                div {
                    class: "mb-8 space-y-4  mb-6 border-b-2 border-red-600",
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
                        "We develop scalable, uncertainty-aware methods to understand and forecast phenomena such as
                        extreme environmental events and renewable energy (wind and solar photovoltaic) dynamics. "
                        i {
                            class: "hover:text-red-700 transition-colors",
                            "To integrate Geoinformatics and AI is not just to interpret the Earth as it is,
                            but to anticipate how it will evolve in the future, both near and far."
                        }
                    }
                }

                // // Research Areas Section
                // div {
                //     class: "mb-12 space-y-6",
                //     h2 {
                //         class: "text-3xl font-bold text-gray-800 mb-6 border-b-2 border-red-600 pb-2",
                //         "Research Areas"
                //     }
                    
                //     div {
                //         class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        
                //         div {
                //             class: "bg-gray-50 p-6 rounded-lg border-l-4 border-red-600",
                //             h3 {
                //                 class: "text-xl font-semibold text-gray-800 mb-3",
                //                 "🌍 Environmental Monitoring"
                //             }
                //             p {
                //                 class: "text-gray-600",
                //                 "Extreme weather events, climate change impacts, and environmental hazard prediction using satellite data and AI models."
                //             }
                //         }
                        
                //         div {
                //             class: "bg-gray-50 p-6 rounded-lg border-l-4 border-yellow-500",
                //             h3 {
                //                 class: "text-xl font-semibold text-gray-800 mb-3",
                //                 "☀️ Renewable Energy"
                //             }
                //             p {
                //                 class: "text-gray-600",
                //                 "Solar and wind energy forecasting, resource assessment, and optimization using geospatial analytics and machine learning."
                //             }
                //         }
                        
                //         div {
                //             class: "bg-gray-50 p-6 rounded-lg border-l-4 border-blue-600",
                //             h3 {
                //                 class: "text-xl font-semibold text-gray-800 mb-3",
                //                 "🗺️ Geospatial AI"
                //             }
                //             p {
                //                 class: "text-gray-600",
                //                 "Developing AI methods that understand spatial relationships, temporal patterns, and uncertainty in geospatial data."
                //             }
                //         }
                        
                //         div {
                //             class: "bg-gray-50 p-6 rounded-lg border-l-4 border-green-600",
                //             h3 {
                //                 class: "text-xl font-semibold text-gray-800 mb-3",
                //                 "📊 Predictive Modeling"
                //             }
                //             p {
                //                 class: "text-gray-600",
                //                 "Uncertainty quantification, ensemble forecasting, and scalable modeling frameworks for Earth system applications."
                //             }
                //         }
                //     }
                // }

                // // Projects Section
                // div {
                //     class: "mb-12 space-y-6",
                //     h2 {
                //         class: "text-3xl font-bold text-gray-800 mb-6 border-b-2 border-red-600 pb-2",
                //         "Current Projects"
                //     }
                    
                //     div {
                //         class: "space-y-6",
                        
                //         div {
                //             class: "bg-white p-6 rounded-lg shadow-md border border-gray-200 hover:shadow-lg transition-shadow",
                //             h3 {
                //                 class: "text-xl font-semibold text-gray-800 mb-3",
                //                 "AI-Powered Weather Forecasting"
                //             }
                //             p {
                //                 class: "text-gray-600 mb-3",
                //                 "Developing machine learning models to improve short-term and medium-term weather prediction accuracy, with focus on extreme event detection and uncertainty quantification."
                //             }
                //             div {
                //                 class: "flex flex-wrap gap-2",
                //                 span { class: "px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm", "Machine Learning" }
                //                 span { class: "px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm", "Weather Prediction" }
                //                 span { class: "px-3 py-1 bg-purple-100 text-purple-800 rounded-full text-sm", "Uncertainty" }
                //             }
                //         }
                        
                //         div {
                //             class: "bg-white p-6 rounded-lg shadow-md border border-gray-200 hover:shadow-lg transition-shadow",
                //             h3 {
                //                 class: "text-xl font-semibold text-gray-800 mb-3",
                //                 "Solar Energy Resource Mapping"
                //             }
                //             p {
                //                 class: "text-gray-600 mb-3",
                //                 "Creating high-resolution solar irradiance maps using satellite data and deep learning to optimize solar panel placement and energy forecasting."
                //             }
                //             div {
                //                 class: "flex flex-wrap gap-2",
                //                 span { class: "px-3 py-1 bg-yellow-100 text-yellow-800 rounded-full text-sm", "Solar Energy" }
                //                 span { class: "px-3 py-1 bg-red-100 text-red-800 rounded-full text-sm", "Remote Sensing" }
                //                 span { class: "px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm", "Deep Learning" }
                //             }
                //         }
                        
                //         div {
                //             class: "bg-white p-6 rounded-lg shadow-md border border-gray-200 hover:shadow-lg transition-shadow",
                //             h3 {
                //                 class: "text-xl font-semibold text-gray-800 mb-3",
                //                 "Geospatial Data Analytics Platform"
                //             }
                //             p {
                //                 class: "text-gray-600 mb-3",
                //                 "Building scalable cloud-based tools for processing and analyzing large-scale geospatial datasets with integrated AI capabilities for environmental monitoring."
                //             }
                //             div {
                //                 class: "flex flex-wrap gap-2",
                //                 span { class: "px-3 py-1 bg-gray-100 text-gray-800 rounded-full text-sm", "Cloud Computing" }
                //                 span { class: "px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm", "GIS" }
                //                 span { class: "px-3 py-1 bg-purple-100 text-purple-800 rounded-full text-sm", "Big Data" }
                //             }
                //         }
                //     }
                // }

                // // Team Section
                // div {
                //     class: "mb-12 space-y-6",
                //     h2 {
                //         class: "text-3xl font-bold text-gray-800 mb-6 border-b-2 border-red-600 pb-2",
                //         "Our Team"
                //     }
                    
                //     p {
                //         class: "text-gray-600 text-lg leading-relaxed",
                //         "Our lab brings together researchers from diverse backgrounds including computer science, 
                //         atmospheric science, geography, and engineering. We foster a collaborative environment 
                //         where interdisciplinary thinking drives innovation in geospatial AI applications."
                //     }
                    
                //     div {
                //         class: "bg-blue-50 p-6 rounded-lg border-l-4 border-blue-500",
                //         p {
                //             class: "text-gray-700 italic",
                //             "\"Our strength lies in combining domain expertise in Earth sciences with cutting-edge 
                //             AI methodologies, enabling us to tackle complex environmental challenges with both 
                //             scientific rigor and technological innovation.\""
                //         }
                //     }
                // }
            }
        }
    }
}
