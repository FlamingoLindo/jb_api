use crate::entities::types;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct TypeResponse {
    pub name: String,
    pub blocked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<types::Model> for TypeResponse {
    fn from(type_data: types::Model) -> Self {
        Self {
            name: type_data.name,
            blocked: type_data.blocked,
            created_at: type_data.created_at,
            updated_at: type_data.updated_at,
        }
    }
}
