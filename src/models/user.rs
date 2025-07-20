use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub image: String,
    pub email: String,
    pub verified_email: bool,
    pub phone: Option<String>,
    pub verified_phone: bool,
    pub password: String,
    pub address: Option<String>,
    pub province_id: Option<i32>,
    pub district_id: Option<i32>,
    pub ward_id: Option<i32>,
    pub address_type: Option<bool>,
    pub address_full: Option<String>,
    pub method_ship: Option<bool>,
    pub date_birth: Option<String>,
    pub sex: Option<String>,
    pub facebook_id: Option<String>,
    pub google_id: Option<String>,
    pub deleted: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}
