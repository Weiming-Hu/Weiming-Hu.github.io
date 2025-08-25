use dioxus::prelude::*;
use super::header::Header;
use super::page_main::Home;
use super::page_pub::Pub;
use super::page_code::Code;
use super::page_info::Info;
use super::page_gaim::Lab;
use super::page_cal::Cal;
use super::page_res::Resources;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]

    #[route("/")]
    Home {},

    #[route("/:pagename")]
    Director { pagename: String },
}

#[component]
pub fn Director(pagename: String) -> Element {
    match pagename.as_str() {
        "home" => rsx!(Home {}),
        "pub" => rsx!(Pub {}),
        "code" => rsx!(Code {}),
        "info" => rsx!(Info {}),
        "gaim" => rsx!(Lab {}),
        "res" => rsx!(Resources {}),
        "meet" => rsx!(Cal {}),
        _ => rsx!(Home {}),
    }
}