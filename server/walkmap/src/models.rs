use serde::{Deserialize, Serialize};

use crate::schema::maps;
use crate::schema::users;

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
pub struct Map {
    pub id: String,
    pub user_id: Option<String>,
    pub created_at: i64,
}
