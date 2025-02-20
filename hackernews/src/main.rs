use dioxus::prelude::*;

// #[derive(Debug, Clone, Routable, PartialEq)]
// #[rustfmt::skip]
// enum Route {
//     #[layout(Navbar)]
//     #[route("/")]
//     Home {},
//     #[route("/blog/:id")]
//     Blog { id: i32 },
// }

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        document::Stylesheet {
            href: asset!( "/assets/main.css" )
        }
        document::Stylesheet {
            href: asset!( "/assets/tailwind.css" )
        }

        body {
            button {
                class: "focus:outline-none text-white bg-purple-700 hover:bg-purple-800 focus:ring-4 focus:ring-purple-300 font-medium rounded-lg text-sm px-5 py-2.5 mb-2 dark:bg-purple-600 dark:hover:bg-purple-700 dark:focus:ring-purple-900",
                "Click me"
            }

            div {
                class: "text-center text-40px",
                "abcd"
            }

            a {
                href: "#",
                class: "block max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow-sm hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700",

                h5 {
                    class: "mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white",
                    "Noteworthy technology acquisitions 2021"
                }

                p {
                    class: "font-normal text-gray-700 dark:text-gray-400",
                    "Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order."
                }
            }
        }


    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hackernews_should_work() {}
}
