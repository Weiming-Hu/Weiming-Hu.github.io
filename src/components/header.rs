use dioxus::prelude::*;
use super::routes::Route;

const BG_IMG_LIGHT: Asset = asset!("/assets/imgs/bg_light.jpg");
const CSS_PAGE_SELECTED: &str = "text-white hover:text-gray-200 font-extrabold";
const CSS_PAGE_NORMAL: &str = "text-white hover:text-gray-200 font-normal";

#[component]
pub fn Header() -> Element {
    let current_route = use_route::<Route>();
    rsx! {
        div {
            class: "min-h-screen relative",
            
            div {
                class: "fixed inset-0",
                style: format!(
                    "background-image: url('{}'); background-size: cover; background-position: center; width: 100vw; height: 100vh;",
                    BG_IMG_LIGHT,
                ),
            }

            header {
                class: "w-full p-2 flex justify-center fixed top-0 left-0 z-50 bg-gray-600/95",
                nav {
                    class: "flex space-x-6 text-lg",
                    Link { 
                        to: Route::Home {}, 
                        class: if matches!(current_route, Route::Home {}) {
                            CSS_PAGE_SELECTED
                        } else {
                            CSS_PAGE_NORMAL
                        },
                        "Home" 
                    }
                    Link { 
                        to: Route::Director { pagename: "gaim".to_string() },
                        class: {
                            let is_active = match &current_route {
                                Route::Director { pagename } if pagename == "gaim" => true,
                                _ => false,
                            };
                            if is_active {
                                CSS_PAGE_SELECTED
                            } else {
                                CSS_PAGE_NORMAL
                            }
                        },
                        "Lab" 
                    }
                    // Link { 
                    //     to: Route::Director { pagename: "pub".to_string() },
                    //     class: {
                    //         let is_active = match &current_route {
                    //             Route::Director { pagename } if pagename == "pub" => true,
                    //             _ => false,
                    //         };
                    //         if is_active {
                    //             CSS_PAGE_SELECTED
                    //         } else {
                    //             CSS_PAGE_NORMAL
                    //         }
                    //     },
                    //     "Pubs" 
                    // }
                    // Link {
                    //     to: Route::Director { pagename: "code".to_string() },
                    //     class: {
                    //         let is_active = match &current_route {
                    //             Route::Director { pagename } if pagename == "code" => true,
                    //             _ => false,
                    //         };
                    //         if is_active {
                    //             CSS_PAGE_SELECTED
                    //         } else {
                    //             CSS_PAGE_NORMAL
                    //         }
                    //     },
                    //     "Code"
                    // }
                    Link {
                        to: Route::Director { pagename: "res".to_string() },
                        class: {
                            let is_active = match &current_route {
                                Route::Director { pagename } if pagename == "res" => true,
                                _ => false,
                            };
                            if is_active {
                                CSS_PAGE_SELECTED
                            } else {
                                CSS_PAGE_NORMAL
                            }
                        },
                        "Resources"
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}
