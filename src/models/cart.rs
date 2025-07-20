use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct Carts {
    pub id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub user_guest_id: Option<uuid::Uuid>,
    pub status: Option<String>,
    pub active: Option<bool>,
    pub deleted: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct CartItems {
    pub cart_id: uuid::Uuid,
    pub product_variants_id: i32,
    pub quantity: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
