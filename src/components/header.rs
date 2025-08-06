use dioxus::prelude::*;
use super::routes::Route;

const BG_IMG_LIGHT: Asset = asset!("/assets/imgs/bg_light.jpg");

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
                class: "w-full p-2 flex justify-center fixed top-0 left-0 z-50 bg-white bg-opacity-90 shadow",
                nav {
                    class: "flex space-x-6 text-sm",
                    Link { 
                        to: Route::Home {}, 
                        class: if matches!(current_route, Route::Home {}) {
                            "text-red-700 hover:text-red-900 font-extrabold"
                        } else {
                            "text-red-700 hover:text-red-900 font-normal"
                        },
                        "Main" 
                    }
                    Link { 
                        to: Route::Director { pagename: "pub".to_string() },
                        class: {
                            let is_active = match &current_route {
                                Route::Director { pagename } if pagename == "pub" => true,
                                _ => false,
                            };
                            if is_active {
                                "text-red-700 hover:text-red-900 font-extrabold"
                            } else {
                                "text-red-700 hover:text-red-900 font-normal"
                            }
                        },
                        "Pubs" 
                    }
                    Link {
                        to: Route::Director { pagename: "code".to_string() },
                        class: {
                            let is_active = match &current_route {
                                Route::Director { pagename } if pagename == "code" => true,
                                _ => false,
                            };
                            if is_active {
                                "text-red-700 hover:text-red-900 font-extrabold"
                            } else {
                                "text-red-700 hover:text-red-900 font-normal"
                            }
                        },
                        "Code"
                    }
                    // a { 
                    //     href: "#", 
                    //     class: "text-gray-700 hover:text-gray-900",
                    //     "CV" 
                    // }
                    // a { 
                    //     href: "#", 
                    //     class: "text-gray-700 hover:text-gray-900",
                    //     "Blog" 
                    // }
                }
            }
            Outlet::<Route> {}
        }
    }
}
