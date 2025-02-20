use anyhow::Result;
use futures::future::join_all;

use crate::{Comment, StoryData, StoryItem};

#[allow(unused)]
const MAX_STORIES: usize = 50;

#[allow(unused)]
pub async fn get_top_stories(mut max_size: usize) -> Result<Vec<StoryItem>> {
    max_size = max_size.min(MAX_STORIES);
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let ids = reqwest::get(url).await?.json::<Vec<i64>>().await?;
    let story_futures = ids.into_iter().take(max_size).map(get_story_item_by_id);
    let items = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|item| item.ok())
        .collect::<Vec<StoryItem>>();

    Ok(items)
}

#[allow(unused)]
pub async fn get_story_item_by_id(id: i64) -> Result<StoryItem> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
    let item = reqwest::get(&url).await?.json::<StoryItem>().await?;
    Ok(item)
}

#[allow(unused)]
pub async fn get_story_comments(item: StoryItem) -> Result<StoryData> {
    let comment_futures = item.kids.iter().map(|id| get_comment_by_id(*id));
    let comments = join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(|comment| comment.ok())
        .collect::<Vec<Comment>>();

    Ok(StoryData { item, comments })
}

#[allow(unused)]
pub async fn get_comment_by_id(comment_id: i64) -> Result<Comment> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{comment_id}.json");
    let comment = reqwest::get(&url).await?.json::<Comment>().await?;
    Ok(comment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_top_stories_should_work() {
        let stories = get_top_stories(3).await.unwrap();
        assert_eq!(stories.len(), 3);
    }

    #[tokio::test]
    async fn get_comment_by_id_should_work() {
        let story = get_top_stories(1).await.unwrap().pop().unwrap();
        let id = story.kids[0];
        let comment = get_comment_by_id(id).await.unwrap();
        assert_eq!(comment.id, id);
    }
}
