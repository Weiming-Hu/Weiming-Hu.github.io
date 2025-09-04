use dioxus::prelude::*;
use super::css_preset::*;

#[component]
pub fn Cal() -> Element {
    
    use_effect(|| {
        // Inject Cal.com embed script if not already present
        let document = gloo::utils::document();
        let script_id = "cal-embed-script";
        if document.get_element_by_id(script_id).is_none() {
            let script = document.create_element("script").unwrap();
            script.set_attribute("type", "text/javascript").unwrap();
            script.set_attribute("id", script_id).unwrap();
            script.set_inner_html(r#"
                (function (C, A, L) { let p = function (a, ar) { a.q.push(ar); }; let d = C.document; C.Cal = C.Cal || function () { let cal = C.Cal; let ar = arguments; if (!cal.loaded) { cal.ns = {}; cal.q = cal.q || []; d.head.appendChild(d.createElement("script")).src = A; cal.loaded = true; } if (ar[0] === L) { const api = function () { p(api, arguments); }; const namespace = ar[1]; api.q = api.q || []; if(typeof namespace === "string"){cal.ns[namespace] = cal.ns[namespace] || api;p(cal.ns[namespace], ar);p(cal, ["initNamespace", namespace]);} else p(cal, ar); return;} p(cal, ar); }; })(window, "https://app.cal.com/embed/embed.js", "init");
                Cal("init", "meet", {origin:"https://app.cal.com"});
                Cal.ns.meet("ui", {"hideEventTypeDetails":false,"layout":"week_view"});
            "#);
            document.body().unwrap().append_child(&script).unwrap();
        }
        // Set custom data attributes on the button after mount
        if let Some(button) = document.get_element_by_id("cal-meet-btn") {
            button.set_attribute("data-cal-link", "huweiming/meet").ok();
            button.set_attribute("data-cal-namespace", "meet").ok();
            button.set_attribute("data-cal-config", "{\"layout\":\"week_view\"}").ok();
        }
    });

    rsx! {
        div {
            class: format!("{} relative z-10", CSS_CONTENT_CONTAINER),
            div {
                class: CSS_CONTENT_CARD,
                div {
                    class: "mb-12 space-y-6",
                    h2 {
                        class: CSS_PAGE_TITLE,
                        "Busy / Available"
                    }
                    p {
                        class: "text-gray-600 text-lg leading-relaxed",
                        "I share my calendar here with the hope that technologies can make"
                        " our lives easier. Curious how this is done? Check out "
                        a {
                            href: "https://cal.com/",
                            target: "_blank",
                            class: "underline hover:text-red-900 transition-colors",
                            "Cal.com"
                        }
                        "."
                    }
                    p {
                        class: "text-gray-600 text-lg leading-relaxed",
                        b {
                            "You do not have to book from here."
                        }
                        " Simply find an available time period and send your calendar invite to "
                        i {
                            class: "hover:text-red-700 transition-colors",
                            "weiming@uga.edu"
                        }
                        " with topics/agenda and your preferred meeting options, e.g., "
                        "room # for in-person meetings (if my office, GGY 312) or Google/Zoom/Team links. "
                        "If you book from here, only 30-minute meetings are allowed."
                    }
                    p{
                        class: "text-gray-600 text-lg leading-relaxed",
                        b { 
                            "If it is urgent and no available time can be found",
                         }
                        ", please reach "
                        "out to me directly so we can figure out something else."
                    }
                    p {
                        class: "text-gray-600 text-lg leading-relaxed",
                        b {
                            "The calendar updates itself after any scheduling changes."
                        }
                        " So you might see your booked slots turn busy right after you send me the invite. "
                        "I will decline any meetings if I cannot make it but I will make sure to communicate this with you."
                    }
                    // Cal.com element-click embed button
                    button {
                        id: "cal-meet-btn",
                        class: "block mb-4 p-4 border-l-4 border-red-400 rounded-r-lg bg-white hover:bg-gray-50 hover:shadow-lg hover:border-red-500 transition-all duration-200 cursor-pointer transform hover:-translate-y-1",

                        p { 
                            class: "text-lg font-semibold text-gray-900 hover:text-red-700 transition-colors",
                            "See My Busy/Available Times"
                         }
                    }
                    
                }
            }
        }
    }
}
