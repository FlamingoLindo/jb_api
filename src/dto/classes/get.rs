use crate::entities::classes;
use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ClassResponse {
    pub name: String,
    pub blocked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<classes::Model> for ClassResponse {
    fn from(class: classes::Model) -> Self {
        Self {
            name: class.name,
            blocked: class.blocked,
            created_at: class.created_at,
            updated_at: class.updated_at,
        }
    }
}
