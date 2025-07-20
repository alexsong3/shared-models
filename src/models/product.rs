use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct Products {
    pub id: i32,
    pub image: String,
    pub user_id: uuid::Uuid,
    pub status: String,
    pub status_product: String,
    pub priority: i32,
    pub is_collection: bool,
    pub focus: bool,
    pub number_view: i32,
    pub deleted: bool,
    #[serde(rename = "publishDate")]
    pub publish_date: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductsLanguage {
    pub product_id: i32,
    pub language_id: i32,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub html_content: Option<String>,
    pub info: Option<String>,
    pub help: Option<String>,
    pub policy: Option<String>,
    pub note: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductsVariants {
    pub id: i32,
    pub product_id: i32,
    pub frame_id: i32,
    pub size_id: i32,
    pub original_price: i32,
    pub discount_price: i32,
    pub final_price: i32,
    pub discount_rate: i32,
    pub priority_frame: i32,
    pub priority_size: i32,
    pub status: String,
    pub status_product: String,
    pub deleted: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductsImages {
    pub id: i32,
    pub product_id: i32,
    pub frame_id: i32,
    pub image_url: String,
    pub priority: Option<i32>,
    pub is_primary: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct ProductsCategory {
    pub id: i32,
    pub group_id: i32,
    pub priority: i32,
    pub active: bool,
    pub deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct ProductsCategoryLanguage {
    pub product_category_id: i32,
    pub language_id: i32,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct ProductsForCategory {
    pub product_id: i32,
    pub product_category_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductsComment {
    pub id: i32,
    pub users_id: i32,
    pub posts_id: i32,
    pub group_id: i32,
    pub content: String,
    pub image: String,
    pub deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
