use dioxus_logger::tracing::Level;
use hackernews::App;

// use dioxus::prelude::*;

// #[derive(Debug, Clone, Routable, PartialEq)]
// #[rustfmt::skip]
// enum Route {
//     #[route("/")]
//     Home {},
//     #[route("/blog/:id")]
//     Blog { id: i32 },
//     #[route("/:..route")]
//     NotFound { route: Vec<String>},
// }

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}
