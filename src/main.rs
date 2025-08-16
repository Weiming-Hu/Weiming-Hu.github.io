mod components;

use dioxus::prelude::*;
use components::Route;

const FAVICON: Asset = asset!("/assets/icons/favicon_uga.ico");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind_output.css");
const FA_CSS: &str = "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(move || {
        document::eval(
            r#"
            document.body.classList.add('loaded');
            "#
        );
    });

    use_effect(|| {
        let document = web_sys::window().unwrap().document().unwrap();
        let script = document.create_element("script").unwrap();
        script.set_attribute("data-collect-dnt", "true").unwrap();
        script.set_attribute("async", "").unwrap();
        script.set_attribute("src", "https://scripts.simpleanalyticscdn.com/latest.js").unwrap();
        document.body().unwrap().append_child(&script).unwrap();
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: FA_CSS }
        document::Link { rel: "icon", href: FAVICON }
        
        Router::<Route> {}
    }
}
