use serde::{Deserialize, Serialize};
use serde_json::Number;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone, PartialEq, Eq)]
pub struct Language {
    pub id: Number,
    pub name: String,
    pub key: String,
}
