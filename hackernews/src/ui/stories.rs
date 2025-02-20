#![allow(non_snake_case)]

use super::CommentsState;
use crate::{
    api::{get_story_comments, get_top_stories},
    StoryData, StoryItem as StoryItemData,
};
use dioxus::prelude::*;

#[component]
pub fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(20));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
          ul {
            for story in stories {
              StoryItem { story: story.clone() }
            }
          }
        },
        Some(Err(err)) => rsx! {
          div { class: "mt-6 text-red-500",
            p { "Failed to fetch stories" }
            p { "{err}" }
          }
        },
        None => rsx! {
          div { class: "mt-6",
            p { "Loading stories..." }
          }
        },
    }
}

#[component]
pub fn StoryItem(story: StoryItemData) -> Element {
    let comments_state = use_context::<Signal<CommentsState>>();
    // cache of the already loaded comments: Option<StoryData>
    let full_story = use_signal(|| None);
    rsx! {
      li { class: "px-3 py-5 transition border-b hover:bg-indigo-100",
        a { href: "#", class: "flex items-center justify-between",
          h3 { class: "text-lg font-semibold", "{story.title}" }
          p { class: "text-gray-400 text-md" }
        }
        div { class: "italic text-gray-400 text-md",
          span { "{story.score} points by {story.by} {story.time} | " }
          a {
            href: "#",
            onclick: move |event| {
                event.prevent_default();
                load_comments(comments_state, full_story, story.clone())
            },
            "{story.kids.len()} comments"
          }
        }
      }
    }
}

async fn load_comments(
    mut comments_state: Signal<CommentsState>,
    mut full_story: Signal<Option<StoryData>>,
    story: StoryItemData,
) {
    // if the comments are already loaded, just change comments_state and return
    if let Some(story_data) = full_story.as_ref() {
        *comments_state.write() = CommentsState::Loaded(story_data.clone());
        return;
    }

    // if no, set comments_state to Loading and fetch the comments
    *comments_state.write() = CommentsState::Loading;

    if let Ok(story_data) = get_story_comments(story).await {
        *comments_state.write() = CommentsState::Loaded(story_data.clone());
        *full_story.write() = Some(story_data);
    }
}
