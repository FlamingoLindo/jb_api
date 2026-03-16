use serde::Serialize;

#[derive(Serialize)]
pub struct TypeResponse {
    pub name: String,
}
#[derive(Serialize)]
pub struct ClassResponse {
    pub name: String,
}
#[derive(Serialize)]
pub struct BrandResponse {
    pub name: String,
    pub image: String,
}
