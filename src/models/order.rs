use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[sqlx(type_name = "pay_type", rename_all = "lowercase")] // Đảm bảo tên khớp với PostgreSQL
pub enum PayType {
    #[serde(rename = "cod")]
    Cod,   // Thanh toán khi nhận hàng
    #[serde(rename = "vnpay")]
    Vnpay, // Thanh toán qua VNPay
    #[serde(rename = "payos")]
    Payos,
    #[serde(rename = "bank_transfer")]
    BankTransfer,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Success,
    Cancel,
    Failed,
    Refunded,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct Orders {
    pub id: uuid::Uuid,
    pub order_code: i64,
    pub user_id: Option<uuid::Uuid>,
    pub user_guest_id: Option<uuid::Uuid>,
    pub cart_id: uuid::Uuid,
    pub pay_type: PayType, // 🔹 Đổi từ String -> PayType
    pub status_pay: OrderStatus,
    pub status_order: OrderStatus,
    pub status_delivery: OrderStatus,
    pub transport_type: bool,
    pub total_quantity: i32,
    pub total_original_price: i32,
    pub total_discount_price: i32,
    pub total_final_price: i32,
    pub active: bool,
    pub deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct OrderItems {
    pub order_id: uuid::Uuid,
    pub product_variants_id: i32,
    pub product_id: i32,
    pub product_title: Option<String>,
    pub product_image: Option<String>,
    pub frame_id: Option<i32>,
    pub frame_title: Option<String>,
    pub size_id: Option<i32>,
    pub size_title: Option<String>,
    pub quantity: Option<i32>,
    pub original_price: Option<i32>,
    pub discount_price: Option<i32>,
    pub final_price: Option<i32>,
    pub discount_rate: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct OrderUserInfo {
    pub id: i32,
    pub order_id: uuid::Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub sex: Option<String>,
    pub address: Option<String>,
    pub province_id: Option<i32>,
    // pub province_name: String,
    pub district_id: Option<i32>,
    // pub district_name: String,
    pub ward_id: Option<i32>,
    // pub ward_name: String,
    pub address_full: Option<String>,
    pub address_type: Option<bool>,
}
