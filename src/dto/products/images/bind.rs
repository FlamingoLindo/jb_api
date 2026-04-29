use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct BindImageProductDTO {
    pub prod_id: Uuid,
    pub img_id: Uuid,
}
