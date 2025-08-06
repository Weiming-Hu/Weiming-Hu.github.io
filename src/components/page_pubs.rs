use dioxus::prelude::*;
use super::css_preset::*;

const CSS_HLINE: &str = "border-b border-gray-200 pb-6";
const CSS_YEAR: &str = "text-2xl font-bold text-red-900 mb-4";

#[component]
pub fn Pubs() -> Element {
    rsx! {
        div {
            class: CSS_CONTENT_CONTAINER,
            
            div {
                class: CSS_CONTENT_CARD,
                
                // Header
                div {
                    class: "mb-8",
                    h1 {
                        class: CSS_PAGE_TITLE,
                        "Publications"
                    }
                    p {
                        class: "text-gray-600 text-lg",
                        "Selected research publications and contributions below. "
                        
                        a { 
                            href: "https://scholar.google.com/citations?user=2xypOLMAAAAJ&hl",
                            target: "_blank",
                            class: "mb-2 hover:text-red-700 transition-colors",
                            "My full profile is on Google Scholar."
                        }
                    }
                }

                // Publications List
                div {
                    class: "space-y-6",
                    
                    // 2025 Publications
                    div {
                        class: CSS_HLINE,
                        h2 {
                            class: CSS_YEAR,
                            "2025"
                        }

                        PublicationEntry { 
                            title: "Global daily discharge estimation based on grid long short‐term memory (LSTM) model and river routing",
                            authors: "Yang, Y., Feng, D., Beck, H. E., Hu, W., Abbas, A., Sengupta, A., ... & Pan, M.",
                            journal: "Water Resources Research",
                            volume: "61(6)",
                            pages: "",
                            year: "2025",
                            website: "https://doi.org/10.1029/2024WR039764"
                        }

                        PublicationEntry {  
                            title: "Toward calibrated ensembles of neural weather model forecasts",
                            authors: "Baño‐Medina, J., Sengupta, A., Watson‐Parris, D., Hu, W., & Delle Monache, L",
                            journal: "Journal of Advances in Modeling Earth Systems",
                            volume: "17(4)",
                            pages: "",
                            year: "2025",
                            website: "https://doi.org/10.1029/2024MS004734"
                        }
                    }

                    // 2024 Publications
                    div {
                        class: CSS_HLINE,
                        h2 {
                            class: CSS_YEAR,
                            "2024"
                        }
                        
                        PublicationEntry {
                            title: "Predicting particulate matter (PM10) levels in Morocco: a 5-day forecast using the analog ensemble method",
                            authors: "Houdou, A., Khomsi, K., Delle Monache, L., Hu, W., Boutayeb, S., Belyamani, L., Abdulla, F., Al-Delaimy, W. K., & Khalis, M.",
                            journal: "Environmental Monitoring and Assessment",
                            volume: "197(1)",
                            pages: "1-20",
                            year: "2024",
                            website: "https://link.springer.com/article/10.1007/s10661-024-13434-z"
                        }
                        
                        PublicationEntry {
                            title: "Arctic accessibility: recent trend in observed ship tracks and validation of arctic transport accessibility model",
                            authors: "Hu, W., Cervone, G., Trusel, L., & Yu, M.",
                            journal: "Annals of GIS",
                            volume: "",
                            pages: "1-20",
                            year: "2024",
                            website: "https://doi.org/10.1080/19475683.2024.2380678"
                        }

                        PublicationEntry {
                            title: "Deep Learning of a 200-member Ensemble with a Limited Historical Training to Improve the Prediction of Extreme Precipitation Events",
                            authors: "Ghazvinian, M., Delle Monache, L., Afzali Gorooh, V., Steinhoff, D., Sengupta, A., Hu, W., Simpson, M., Weihs, R., Papadopoulos, C., Mulrooney, P., et al.",
                            journal: "Monthly Weather Review",
                            volume: "",
                            pages: "",
                            year: "2024",
                            website: "https://doi.org/10.1175/MWR-D-23-0277.1"
                        }

                        PublicationEntry {
                            title: "People and Data: solving planetary challenges together",
                            authors: "Vanalli, C., Howerton, E., Yang, F., Tran, T. N., & Hu, W.",
                            journal: "Frontiers in Environmental Science",
                            volume: "12",
                            pages: "1332844",
                            year: "2024",
                            website: "https://doi.org/10.3389/fenvs.2024.1332844"
                        }
                    }

                    // 2023 Publications
                    div {
                        class: CSS_HLINE,
                        h2 {
                            class: CSS_YEAR,
                            "2023"
                        }
                        
                        PublicationEntry {
                            title: "Theory of spatiotemporal deep analogs and their application to solar forecasting",
                            authors: "Hu, W., Cervone, G., & Young, G.",
                            journal: "Artificial Intelligence in Earth Science",
                            volume: "",
                            pages: "205-246",
                            year: "2023",
                            website: "https://doi.org/10.1016/B978-0-323-91737-7.00005-0"
                        }

                        PublicationEntry {
                            title: "Deep Learning Forecast Uncertainty for Precipitation over Western US",
                            authors: "Hu, W., Ghazvinian, M., Chapman, W. E., Sengupta, A., Ralph, F. M., & Delle Monache, L.",
                            journal: "Monthly Weather Review",
                            volume: "",
                            pages: "",
                            year: "2023",
                            website: "https://doi.org/10.1175/MWR-D-22-0268.1"
                        }

                        PublicationEntry {
                            title: "Machine Learning Weather Analogs for Near-Surface Variables",
                            authors: "Hu, W., Cervone, G., Young, G., & Delle Monache, L.",
                            journal: "Boundary-Layer Meteorology",
                            volume: "",
                            pages: "1-25",
                            year: "2023",
                            website: "https://link.springer.com/article/10.1007/s10546-022-00779-6"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PublicationEntry(
    title: String,
    authors: String,
    journal: String,
    volume: String,
    pages: String,
    year: String,
    website: String,
) -> Element {
    rsx! {
        if !website.is_empty() {
            a {
                href: "{website}",
                target: "_blank",
                class: "block mb-4 p-4 border-l-4 border-red-400 rounded-r-lg bg-white hover:bg-gray-50 hover:shadow-lg hover:border-red-500 transition-all duration-200 cursor-pointer transform hover:-translate-y-1",
                
                h3 {
                    class: "text-lg font-semibold text-gray-900 mb-2 hover:text-red-700 transition-colors",
                    "{title}"
                }
                
                p {
                    class: "text-gray-700 mb-1",
                    "{authors}"
                }
                
                div {
                    class: "flex flex-wrap items-center gap-2 text-sm text-gray-600",
                    span {
                        class: "font-medium text-red-700 italic",
                        "{journal}"
                    }
                    if !volume.is_empty() {
                        span { ", Vol. {volume}" }
                    }
                    if !pages.is_empty() {
                        span { ", pp. {pages}" }
                    }
                    span {
                        class: "font-medium",
                        " ({year})"
                    }
                }
            }
        } else {
            div {
                class: "mb-4 p-4 border-l-4 border-gray-400 rounded-r-lg bg-gray-50",
                
                h3 {
                    class: "text-lg font-semibold text-gray-900 mb-2",
                    "{title}"
                }
                
                p {
                    class: "text-gray-700 mb-1",
                    "{authors}"
                }
                
                div {
                    class: "flex flex-wrap items-center gap-2 text-sm text-gray-600",
                    span {
                        class: "font-medium text-gray-700 italic",
                        "{journal}"
                    }
                    if !volume.is_empty() {
                        span { ", Vol. {volume}" }
                    }
                    if !pages.is_empty() {
                        span { ", pp. {pages}" }
                    }
                    span {
                        class: "font-medium",
                        " ({year})"
                    }
                }
            }
        }
    }
}
