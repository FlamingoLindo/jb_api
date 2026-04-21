use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct AvailableDTO {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct AvailableQueryParams {
    pub search: Option<String>,
}
