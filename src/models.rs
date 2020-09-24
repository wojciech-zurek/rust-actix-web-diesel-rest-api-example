use serde::{Serialize, Deserialize};
use chrono::Utc;
use uuid::Uuid;
use crate::schema::users;

#[derive(Serialize, Debug, Queryable, PartialEq)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: i64,
    pub public_id: Uuid,
    pub name: String,
    pub create_date: chrono::DateTime<Utc>,
    pub update_date: chrono::DateTime<Utc>,
}

#[derive(Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: String
}