use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use sqlx::Type;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "users_status")] // Tên kiểu enum trong PostgreSQL
#[sqlx(rename_all = "lowercase")] // Ánh xạ các giá trị thành chữ thường
pub enum UserStatus {
    Active,
    Pending,
    Stop,
}

// use crate::models::user_admin::UserStatus;
// r#"
//         INSERT INTO users_admin (name, email, password)
//         VALUES ($1, $2, $3)
//         RETURNING id, name, email, password, role, image, verified, phone, address, sex, status::text as "status: UserStatus", deleted, created_at, updated_at
//         "#,

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct UserAdmin {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub image: String,
    pub verified: bool,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub sex: Option<String>,
    pub status: Option<UserStatus>,
    pub deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserAdminSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserAdminSchema {
    pub email: String,
    pub password: String,
}
