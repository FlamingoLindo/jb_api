use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct AvailableDTO {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct AvailableQueryParams {
    pub search: Option<String>,
}
