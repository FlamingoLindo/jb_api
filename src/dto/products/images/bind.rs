use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct BindImageProductDTO {
    pub prod_id: Uuid,
    pub img_id: Uuid,
}
