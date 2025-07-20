use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct Frames {
    pub id: i32,
    pub image: String,
    pub title: String,
    // pub title_en: String,
    pub description: String,
    // pub description_en: String,
    pub html_content: String,
    // pub html_content_en: Option<String>,
    pub hex_code: String,
    pub active: bool,
    pub priority: i32,
    pub deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct Sizes {
    pub id: i32,
    pub title: String,
    // pub title_en: String,
    pub description: String,
    // pub description_en: String,
    pub active: bool,
    pub priority: i32,
    pub deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct FramesImages {
    pub id: i32,
    pub frame_id: i32,
    pub image_url: String,
    pub is_primary: bool,
    pub priority: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Province {
    pub id: i32,
    pub name: String,
    pub name_en: String,
    pub full_name: String,
    pub full_name_en: String,
    pub code_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct District {
    pub id: i32,
    pub province_id: i32,
    pub name: String,
    pub name_en: String,
    pub full_name: String,
    pub full_name_en: String,
    pub code_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ward {
    pub id: i32,
    pub district_id: i32,
    pub name: String,
    pub name_en: String,
    pub full_name: String,
    pub full_name_en: String,
    pub code_name: Option<String>,
}
