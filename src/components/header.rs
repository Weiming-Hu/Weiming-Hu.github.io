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
                    // a { 
                    //     href: "#", 
                    //     class: "text-gray-700 hover:text-gray-900",
                    //     "Research" 
                    // }
                    // a { 
                    //     href: "#", 
                    //     class: "text-gray-700 hover:text-gray-900",
                    //     "Teaching" 
                    // }
                    Link { 
                        to: Route::Pubs {},
                        class: if matches!(current_route, Route::Pubs {}) {
                            "text-red-700 hover:text-red-900 font-extrabold"
                        } else {
                            "text-red-700 hover:text-red-900 font-normal"
                        },
                        "Pubs" 
                    }
                    Link {
                        to: Route::Code {},
                        class: if matches!(current_route, Route::Code {}) {
                            "text-red-700 hover:text-red-900 font-extrabold"
                        } else {
                            "text-red-700 hover:text-red-900 font-normal"
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
