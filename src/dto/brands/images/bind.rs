use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct BindImageBrandDTO {
    pub brand_id: Uuid,
    pub img_id: Uuid,
}
