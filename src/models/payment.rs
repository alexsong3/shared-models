// use chrono::prelude::*;
// use serde::{Deserialize, Serialize};

// #[allow(non_snake_case)]
// #[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
// pub struct Payment {
//     pub id: i32,
//     pub image: String,
//     pub user_admin_id: uuid::Uuid,
//     pub status: String,
//     pub number_view: i32,
//     pub focus: bool,
//     pub deleted: bool,
//     #[serde(rename = "publishDate")]
//     pub publish_date: Option<DateTime<Utc>>,
//     #[serde(rename = "createdAt")]
//     pub created_at: Option<DateTime<Utc>>,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: Option<DateTime<Utc>>,
// }

// #[derive(Debug, Deserialize)]
// pub struct PaymentLanguage {
//     pub posts_id: i32,
//     pub language_id: i32,
//     pub title: Option<String>,
//     pub slug: Option<String>,
//     pub description: Option<String>,
//     pub html_content: Option<String>,
//     #[serde(rename = "createdAt")]
//     pub created_at: Option<DateTime<Utc>>,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: Option<DateTime<Utc>>,
// }

// #[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
// pub struct PaymentCategory {
//     pub id: i32,
//     pub group_id: i32,
//     pub priority: i32,
//     pub active: bool,
//     pub deleted: bool,
//     #[serde(rename = "createdAt")]
//     pub created_at: Option<DateTime<Utc>>,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: Option<DateTime<Utc>>,
// }

// #[derive(Debug, Deserialize)]
// pub struct PaymentCategoryLanguage {
//     pub posts_category_id: i32,
//     pub language_id: i32,
//     pub title: Option<String>,
//     pub slug: Option<String>,
//     pub description: Option<String>,
//     #[serde(rename = "createdAt")]
//     pub created_at: Option<DateTime<Utc>>,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: Option<DateTime<Utc>>,
// }

// #[derive(Debug, Deserialize)]
// pub struct PaymentForCategory {
//     pub posts_id: i32,
//     pub posts_category_id: i32,
//     #[serde(rename = "createdAt")]
//     pub created_at: Option<DateTime<Utc>>,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: Option<DateTime<Utc>>,
// }

// #[derive(Debug, Deserialize)]
// pub struct PaymentComment {
//     pub id: i32,
//     pub users_id: i32,
//     pub posts_id: i32,
//     pub group_id: i32,
//     pub content: String,
//     pub image: String,
//     pub deleted: bool,
//     #[serde(rename = "createdAt")]
//     pub created_at: Option<DateTime<Utc>>,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: Option<DateTime<Utc>>,
// }
