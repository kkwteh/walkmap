use serde::{Deserialize, Serialize};

use crate::schema::maps;
use crate::schema::markers;
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
pub struct Marker {
    pub id: String,
    pub map_id: String,
    pub order_parameter: f64,
    pub lat: f64,
    pub lon: f64,
    pub annotation: Option<String>,
    pub image_url: Option<String>,
}

// # Scratch notes from walkmap design

// - map
//   - id "ABCDE"

// - marker
//   - id UUID
//   - map_id "ABCDE"
//   - order_parameter f64
//   # order parameter is average of order parameter behind and ahead
//   # if element is at one end of the list take order param of min / max element -/+ 100
//   - lat f64
//   - lon f64
//   - annotation TEXT
//   - image_url TEXT
// # Update markers async
// # Frontend will have an is_synced attribute
// # Frontend will have an insert marker ahead and insert marker behind buttons
