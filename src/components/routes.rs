use dioxus::prelude::*;
use super::header::Header;
use super::page_main::Home;
use super::page_pubs::Pubs;
use super::page_code::Code;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]

    #[route("/")]
    Home {},

    #[route("/pubs")]
    Pubs {},

    #[route("/code")]
    Code {},
}