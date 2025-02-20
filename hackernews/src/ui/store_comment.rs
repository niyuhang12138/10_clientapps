use dioxus::prelude::*;

use crate::StoryItem;

#[component]
pub fn StoryCommentComponent(data: StoryItem) -> Element {
    rsx! {
      li {
        article { class: "mt-8 leading-7 tracking-wider text-gray-500",
            p { "Hi Akhil," }
            p {
                "Design and develop enterprise-facing UI and consumer-facing UI as well as\r\n      REST API\r\n      backends.Work with\r\n      Product Managers and User Experience designers to create an appealing user experience for desktop web and\r\n      mobile web."
            }
            footer { class: "mt-12",
                p { "Thanks & Regards," }
                p { "Alexandar" }
            }
        }
      }
    }
}
