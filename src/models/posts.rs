use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct Posts {
    pub id: i32,
    pub image: String,
    pub cover_photo: Option<String>,
    pub person_write: uuid::Uuid,
    pub status: String,
    pub focus: Option<bool>,
    pub type_posts: Option<String>,
    pub number_view: Option<i32>,
    pub deleted: Option<bool>,
    #[serde(rename = "publishDate")]
    pub publish_date: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostsLanguage {
    pub posts_id: i32,
    pub language_id: i32,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub html_content: Option<String>,
    pub deeplink: Option<String>,
    pub image_description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone, PartialEq, Eq)]
pub struct PostsCategory {
    pub id: i32,
    pub group_id: Option<i32>,
    pub priority: Option<i32>,
    pub active: Option<bool>,
    pub deleted: Option<bool>,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct PostsCategoryLanguage {
    pub posts_category_id: i32,
    pub language_id: i32,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    #[sqlx(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    #[sqlx(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PostsForCategory {
    pub posts_id: i32,
    pub posts_category_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

// #[derive(Debug, Deserialize)]
// pub struct PostsComment {
//     pub id: i32,
//     pub users_id: i32,
//     pub posts_id: i32,
//     pub group_id: Option<i32>,
//     pub content: String,
//     pub image: Option<String>,
//     pub deleted: Option<bool>,
//     #[serde(rename = "createdAt")]
//     pub created_at: Option<DateTime<Utc>>,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: Option<DateTime<Utc>>,
// }
