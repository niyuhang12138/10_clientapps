mod store_comment;
mod story_item;

pub use store_comment::StoryCommentComponent;
pub use story_item::StoryItemComponent;

use dioxus::prelude::*;

use crate::get_top_stories;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet {
            href: asset!( "assets/tailwind.css" )
        }

        main { class: "flex w-full h-full shadow-lg rounded-3xl",
            section { class: "flex flex-col w-4/12 h-full pt-3 overflow-y-scroll bg-gray-50",
                Stories {  }
            }
            section { class: "flex flex-col w-8/12 px-4 bg-white rounded-r-3xl",
                section {
                    ul {}
                }
            }
        }

    }
}

#[component]
fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(20));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
          ul {
            class: "mt-6",
            for story in stories {
              StoryItemComponent {
                story: story.clone()
              }
            }
          }
        },
        Some(Err(err)) => rsx! {
          div {
            class: "mt-6 text-red-500",
            p {
              "Failed to fetch stories"
            }
            p {
              "{err}"
            }
          }
        },
        None => rsx! {
          div {
            class: "mt-6 text-gray-500",
            p {
              "Fetching stories..."
            }
          }
        },
    }
}
