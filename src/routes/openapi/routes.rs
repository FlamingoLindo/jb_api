use crate::dto::brands::create::{CreateBrandDTO, CreateBrandResponse};
use crate::dto::brands::get_all::{BrandsQueryParams, BrandsSortOrder, GetBrandsDTO};
use crate::dto::brands::images::bind::BindImageBrandDTO;
use crate::dto::brands::update::{UpdateBrandDTO, UpdateBrandResponse};
use crate::dto::budget::create::CreateBudgetDTO;
use crate::dto::budget::get_per_client::{
    GetAllBudgetsPerClientDTO, GetAllBudgetsPerClientQueryParams,
};
use crate::dto::clients::available::{AvailableDTO, AvailableQueryParams};
use crate::dto::clients::create::{CreateClientDTO, CreateClientResponse};
use crate::dto::clients::get::ClientResponse;
use crate::dto::clients::get_all::{ClientsQueryParams, ClientsSortOrder, GetClientsDTO};
use crate::dto::clients::update::{UpdateClientDTO, UpdateClientResponse};
use crate::dto::products::create::{CreateProductDTO, CreateProductResponse};
use crate::dto::products::get::ProductResponse;
use crate::dto::products::get_all::{GetProductsDTO, ProductsQueryParams, ProductsSortOrder};
use crate::dto::products::images::bind::BindImageProductDTO;
use crate::dto::products::readjust_price::ReadjustPriceDTO;
use crate::dto::products::update::{UpdateProductDTO, UpdateProductResponse};
use crate::dto::types::create::{CreateTypeDTO, CreateTypeResponse};
use crate::dto::types::get::TypeResponse;
use crate::dto::types::get_all::{GetTypesDTO, TypesQueryParams, TypesSortOrder};
use crate::dto::types::update::{UpdateTypeDTO, UpdateTypeResponse};
use crate::dto::users::create_user::{CreateUserDTO, CreateUserResponseDTO};
use crate::dto::users::forgot_password::ForgotPasswordDTO;
use crate::dto::users::login_user::LoginDTO;
use crate::dto::users::reset_password::ResetPasswordDTO;
use crate::dto::users::send_forgot_password::SendForgotPasswordDTO;
use crate::dto::users::update::{UpdateUserDTO, UpdateUserResponse};
use crate::handlers::brands::block::BlockBrandResponse;
use crate::handlers::clients::block::BlockClientResponse;
use crate::handlers::products::block::BlockProductResponse;
use crate::handlers::types::block::BlockTypeResponse;
use crate::handlers::users::block::BlockUserResponse;
use actix_web::{HttpResponse, get};
use scalar_api_reference::scalar_html;
use serde_json::json;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "JB API", version = "1.0.0"),
    paths(
        // Brands
        crate::handlers::brands::create::create_brand,
        crate::handlers::brands::get::get_brand,
        crate::handlers::brands::get_all::get_brands,
        crate::handlers::brands::update::update_brand,
        crate::handlers::brands::delete::delete_brand,
        crate::handlers::brands::block::block_brand,
        crate::handlers::brands::images::bind::bind_brand_to_image,
        crate::handlers::brands::images::delete::delete_brand_bind,
        // Clients
        crate::handlers::clients::create::create_client,
        crate::handlers::clients::get::get_client,
        crate::handlers::clients::get_all::get_clients,
        crate::handlers::clients::update::update_client,
        crate::handlers::clients::delete::delete_client,
        crate::handlers::clients::block::block_client,
        crate::handlers::clients::available::available_clients,
        crate::handlers::clients::export::export_clients,
        // Products
        crate::handlers::products::create::create_product,
        crate::handlers::products::get::get_product,
        crate::handlers::products::get_all::get_products,
        crate::handlers::products::update::update_product,
        crate::handlers::products::delete::delete_product,
        crate::handlers::products::block::block_product,
        crate::handlers::products::readjust_price::readjust_price,
        crate::handlers::products::export::export_products,
        crate::handlers::products::images::bind::bind_product_to_image,
        crate::handlers::products::images::delete::delete_product_bind,
        // Types
        crate::handlers::types::create::create_type,
        crate::handlers::types::get::get_type,
        crate::handlers::types::get_all::get_types,
        crate::handlers::types::update::update_type,
        crate::handlers::types::delete::delete_type,
        crate::handlers::types::block::block_type,
        // Users
        crate::handlers::users::create::create_user,
        crate::handlers::users::update::update_user,
        crate::handlers::users::delete::delete_user,
        crate::handlers::users::block::block_user,
        crate::handlers::users::login::login,
        crate::handlers::users::send_forgot_password::send_forgot_password,
        crate::handlers::users::forgot_password::forgot_password,
        crate::handlers::users::reset_password::reset_password,
        crate::handlers::users::export::export_users,
        // Budgets
        crate::handlers::budgets::create::create_budget,
        crate::handlers::budgets::get_per_client::get_all_budgets_per_client,
        crate::handlers::budgets::count::count_client_budgets,
        crate::handlers::budgets::delete::delete_budget,
        // Images
        crate::handlers::images::upload::save_file,
        crate::handlers::images::delete::delete_image,
        // Database
        crate::handlers::database::dump::create_db_dump,
    ),
    components(schemas(
        // Brands DTOs
        CreateBrandDTO, CreateBrandResponse,
        BrandsQueryParams, BrandsSortOrder, GetBrandsDTO,
        UpdateBrandDTO, UpdateBrandResponse,
        BindImageBrandDTO,
        BlockBrandResponse,
        // Clients DTOs
        CreateClientDTO, CreateClientResponse,
        ClientResponse,
        ClientsQueryParams, ClientsSortOrder, GetClientsDTO,
        UpdateClientDTO, UpdateClientResponse,
        AvailableDTO, AvailableQueryParams,
        BlockClientResponse,
        // Products DTOs
        CreateProductDTO, CreateProductResponse,
        ProductResponse,
        ProductsQueryParams, ProductsSortOrder, GetProductsDTO,
        UpdateProductDTO, UpdateProductResponse,
        ReadjustPriceDTO,
        BindImageProductDTO,
        BlockProductResponse,
        // Types DTOs
        CreateTypeDTO, CreateTypeResponse,
        TypeResponse,
        TypesQueryParams, TypesSortOrder, GetTypesDTO,
        UpdateTypeDTO, UpdateTypeResponse,
        BlockTypeResponse,
        // Users DTOs
        CreateUserDTO, CreateUserResponseDTO,
        UpdateUserDTO, UpdateUserResponse,
        LoginDTO,
        SendForgotPasswordDTO,
        ForgotPasswordDTO,
        ResetPasswordDTO,
        BlockUserResponse,
        // Budgets DTOs
        CreateBudgetDTO,
        GetAllBudgetsPerClientDTO, GetAllBudgetsPerClientQueryParams,
    )),
    tags(
        (name = "Brands", description = "Brand management"),
        (name = "Clients", description = "Client management"),
        (name = "Products", description = "Product management"),
        (name = "Types", description = "Product type management"),
        (name = "Users", description = "User management"),
        (name = "Budgets", description = "Budget management"),
        (name = "Images", description = "Image management"),
        (name = "Database", description = "Database operations"),
    )
)]
pub struct ApiDoc;

#[get("/api/v1/openapi.json")]
pub async fn openapi_json() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(ApiDoc::openapi().to_json().unwrap())
}

#[get("/scalar")]
async fn scalar_ui() -> HttpResponse {
    let config = json!({
        "url": "/api/v1/openapi.json",
        "theme": "alternate",
        "layout": "moon"
    });
    let html = scalar_html(&config, None);
    HttpResponse::Ok().content_type("text/html").body(html)
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(openapi_json).service(scalar_ui);
}
