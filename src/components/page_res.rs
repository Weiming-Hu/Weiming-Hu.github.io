use dioxus::prelude::*;
use super::css_preset::*;
use chrono::{NaiveDate, Utc, Duration};
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use csv::ReaderBuilder;

#[derive(Debug, Clone)]
struct ResourceRow {
    caption: String,
    link: String,
    expiration_date: Option<NaiveDate>,
    keywords: Vec<String>,
}

fn parse_keywords(s: &str) -> Vec<String> {
    s.split(',').map(|kw| kw.trim().to_string()).filter(|kw| !kw.is_empty()).collect()
}

fn parse_expiration(s: &str) -> Option<NaiveDate> {
    if s.trim().is_empty() {
        None
    } else {
        NaiveDate::parse_from_str(s.trim(), "%Y/%m/%d").ok()
    }
}

async fn fetch_resources() -> Option<Vec<ResourceRow>> {
    let url = "https://docs.google.com/spreadsheets/d/1y-_hrRYhylnryjiOS1f4SNu_NnMP5j6231Qb8qG-0Zk/export?format=csv";
    let client = Client::new();
    let resp = client.get(url).send().await.ok()?.text().await.ok()?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(resp.as_bytes());
    let mut rows = Vec::new();
    for result in rdr.records() {
        if let Ok(record) = result {
            let caption = record.get(0).unwrap_or("").to_string();
            let expiration_date = parse_expiration(record.get(1).unwrap_or(""));
            let keywords = parse_keywords(record.get(2).unwrap_or(""));
            let link = record.get(3).unwrap_or("").to_string();
            rows.push(ResourceRow { caption, link, expiration_date, keywords });
        }
    }
    Some(rows)
}

fn filter_and_sort_resources(resources: Vec<ResourceRow>, days: i64) -> Vec<ResourceRow> {
    let today = Utc::now().date_naive();
    let cutoff = today + Duration::days(days);
    let mut expiring: Vec<ResourceRow> = resources
        .iter()
        .filter(|r| {
            match r.expiration_date {
                Some(date) => date >= today && date <= cutoff,
                None => false,
            }
        })
        .cloned()
        .collect();
    let mut non_expiring: Vec<ResourceRow> = resources
        .iter()
        .filter(|r| r.expiration_date.is_none())
        .cloned()
        .collect();
    expiring.sort_by(|a, b| a.expiration_date.cmp(&b.expiration_date));
    non_expiring.sort_by(|a, b| a.caption.cmp(&b.caption));
    expiring.extend(non_expiring);
    expiring
}

#[component]
fn ResourceCard(
    caption: String,
    link: String,
    expiration_date: Option<NaiveDate>,
    keywords: Vec<String>,
) -> Element {
    rsx! {
        a {
            href: "{link}",
            target: "_blank",
            class: "block p-6 border border-gray-200 rounded-lg bg-white hover:bg-gray-50 hover:shadow-lg hover:border-red-300 transition-all duration-200 cursor-pointer transform hover:-translate-y-1",
            div {
                class: "flex flex-col md:flex-row items-center justify-between w-full",
                // Caption top/left
                div {
                    class: "w-full md:w-4/5 pr-2 text-center md:text-left mb-2",
                    h3 {
                        class: "text-lg leading-relaxed text-gray-900 hover:text-red-700 transition-colors",
                        "{caption}"
                    }
                }
                // Keywords and expiration below/right
                div {
                    class: "w-full md:w-1/5 flex flex-col items-center md:items-end gap-2 mt-2",
                    div {
                        class: "flex flex-wrap gap-2 justify-center md:justify-end",
                        {
                            keywords.iter().map(|kw| rsx! {
                                span { class: "inline-flex items-center px-2 py-1 text-xs font-medium bg-red-100 text-gray-500 rounded-full", "{kw}" }
                            })
                        }
                    }
                    div {
                        class: "mt-2 text-center md:text-right",
                        if let Some(date) = expiration_date {
                            div { class: "text-xs text-gray-500", "Expires: {date}" }
                        } else {
                            div { class: "text-xs text-gray-500", "Never Expires" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Resources() -> Element {
    let mut resources = use_signal(|| Vec::new());
    let days_to_expire = 180;

    // Fetch and parse resources on mount
    use_effect({
        move || {
            spawn_local(async move {
                if let Some(rows) = fetch_resources().await {
                    let filtered = filter_and_sort_resources(rows, days_to_expire);
                    resources.set(filtered);
                }
            });
        }
    });

    rsx! {
        div {
            class: format!("{} relative z-10", CSS_CONTENT_CONTAINER),
            div {
                class: CSS_CONTENT_CARD,
                div {
                    class: "mb-12 space-y-6",
                    h1 {
                        class: CSS_PAGE_TITLE,
                        "Resources"
                    }
                    p {
                        class: "text-gray-600 text-lg leading-relaxed",
                        "I keep this page updated with resources that might help you with your research and study. "
                        "The full list is public on "
                        a {
                            href: "https://docs.google.com/spreadsheets/d/1y-_hrRYhylnryjiOS1f4SNu_NnMP5j6231Qb8qG-0Zk/edit?usp=sharing",
                            target: "_blank",
                            class: CSS_LINK_TEXT,
                            "Google drive"
                        }
                        {format!(". Here I'm only showing items that either do not expire or expire in {} days.", days_to_expire)}
                        " You should check out the full list to see expired items or those expiring in the far future to get a head start!."
                    }
                    p {
                        class: "text-gray-600 text-lg leading-relaxed",
                        "Obviously, you should reach out to the posting agency if you have questions regarding the program. "
                        "But I'm always open to discussions about ideas and strategies. Best of luck!"
                    }
                }

                {
                    if resources.is_empty() {
                        rsx! { div { class: "text-gray-400 py-8", "Loading..." } }
                    } else {
                        rsx!(
                            div {
                                class: "w-full flex flex-col gap-6 mt-8",
                                {
                                    resources().iter().map(|row| {
                                        rsx! {
                                            ResourceCard {
                                                caption: row.caption.clone(),
                                                link: row.link.clone(),
                                                expiration_date: row.expiration_date,
                                                keywords: row.keywords.clone(),
                                            }
                                        }
                                    })
                                }
                            }
                        )
                    }
                }
            }
        }
    }
}
