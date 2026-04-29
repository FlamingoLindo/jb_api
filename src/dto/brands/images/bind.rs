use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct BindImageBrandDTO {
    pub brand_id: Uuid,
    pub img_id: Uuid,
}
