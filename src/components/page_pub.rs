use super::css_preset::*;
use csv::ReaderBuilder;
use wasm_bindgen_futures::spawn_local;
use dioxus::prelude::*;
use reqwest::Client;

const CSS_HLINE: &str = "border-b border-gray-200 pb-6";
const CSS_YEAR: &str = "text-2xl font-bold text-red-900 mb-4";

#[derive(Debug, Clone)]
struct PubRow {
    year: String,
    title: String,
    authors: String,
    journal: String,
    website: String,
    note: String,
}

async fn fetch_publications() -> Option<Vec<PubRow>> {
    let url: &'static str = "https://docs.google.com/spreadsheets/d/1m9TQHNTgvpRE3wg1F-58ovCSFsEr_MwXllsj9sYdViU/export?format=csv";
    let client: Client = Client::new();
    let resp = client.get(url)
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(resp.as_bytes());
    let mut rows = Vec::new();
    for result in rdr.records() {
        if let Ok(record) = result {
            rows.push(PubRow {
                year: record.get(0).unwrap_or("").to_string(),
                title: record.get(1).unwrap_or("").to_string(),
                authors: record.get(2).unwrap_or("").to_string(),
                journal: record.get(3).unwrap_or("").to_string(),
                website: record.get(4).unwrap_or("").to_string(),
                note: record.get(5).unwrap_or("").to_string(),
            });
        }
    }
    Some(rows)
}

#[component]
fn PublicationEntry(
    year: String,
    title: String,
    authors: String,
    journal: String,
    website: String,
    note: String,
) -> Element {
    rsx! {
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
                if !note.is_empty() {
                    span {
                        class: "ml-2 text-xs text-gray-500 italic",
                        "{note}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Pub() -> Element {
    let mut publications = use_signal(|| Vec::new());

    use_effect({
        move || {
            spawn_local(async move {
                if let Some(rows) = fetch_publications().await {
                    publications.set(rows);
                }
            });
        }
    });

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
                    {
                        if publications().is_empty() {
                            rsx!(div { class: "text-gray-400 py-8", "Loading ..." })
                        } else {
                            let mut years: Vec<String> = publications().iter().map(|p| p.year.clone()).collect();
                            years.sort_by(|a, b| b.cmp(a));
                            years.dedup();
                            rsx! {{
                                years.into_iter().map(|year| {
                                    rsx! {
                                        div {  
                                            class: CSS_HLINE,
                                            h2 { class: CSS_YEAR, "{year}" }
                                            {
                                                publications().iter().filter(|p| p.year == year).map(|p| {
                                                    rsx! {
                                                        PublicationEntry {
                                                            year: p.year.clone(),
                                                            title: p.title.clone(),
                                                            authors: p.authors.clone(),
                                                            journal: p.journal.clone(),
                                                            website: p.website.clone(),
                                                            note: p.note.clone(),
                                                        }
                                                    }
                                                })
                                            }
                                        }
                                    }
                                })
                            }}
                        }
                    }
                }
            }
        }
    }
}

