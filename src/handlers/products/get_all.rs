use actix_web::{HttpResponse, Responder, web};
use migration::Alias; // UNDO HERE IF ERROR
use sea_orm::sea_query::Expr;
use sea_orm::{
    Condition, DatabaseConnection, EntityTrait, JoinType, Order, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait,
};
use serde_json::json;

use crate::{
    dto::products::get_all::{GetProductsDTO, ProductsQueryParams, ProductsSortOrder},
    entities::{brands, classes, images, products, products_images, types},
};
use log::warn;

pub async fn get_products(
    db: web::Data<DatabaseConnection>,
    query: web::Query<ProductsQueryParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(10);

    let condition = match &query.search {
        Some(term) if !term.is_empty() => {
            let pattern = format!("%{}%", term);
            Condition::any()
                .add(Expr::cust_with_values(
                    "unaccent(products.description) ILIKE unaccent($1)",
                    [&pattern],
                ))
                .add(Expr::cust_with_values(
                    "unaccent(products.code) ILIKE unaccent($1)",
                    [&pattern],
                ))
                .add(Expr::cust_with_values(
                    "unaccent(types.name) ILIKE unaccent($1)",
                    [&pattern],
                ))
                .add(Expr::cust_with_values(
                    "unaccent(classes.name) ILIKE unaccent($1)",
                    [&pattern],
                ))
                .add(Expr::cust_with_values(
                    "unaccent(brands.name) ILIKE unaccent($1)",
                    [&pattern],
                ))
        }
        _ => Condition::all(),
    };

    let mut select = products::Entity::find()
        .select_only()
        .columns([
            products::Column::Id,
            products::Column::Code,
            products::Column::Description,
            products::Column::Blocked,
            products::Column::CreatedAt,
            products::Column::UpdatedAt,
        ])
        // Type
        .column_as(types::Column::Name, "type_name")
        .join(JoinType::LeftJoin, products::Relation::Types.def())
        // Class
        .column_as(classes::Column::Name, "class_name")
        .join(JoinType::LeftJoin, products::Relation::Classes.def())
        // Brands - name
        .column_as(brands::Column::Name, "brand_name")
        .join(JoinType::LeftJoin, products::Relation::Brands.def())
        // Brands - image
        .column_as(images::Column::Path, "brand_image")
        .join(JoinType::LeftJoin, brands::Relation::Images.def())
        // TODO get product image
        .join(JoinType::LeftJoin, products::Relation::ProductsImages.def())
        .join_as(
            JoinType::LeftJoin,
            products_images::Relation::Images.def(),
            Alias::new("product_imgs"),
        )
        .column_as(
            Expr::col((Alias::new("product_imgs"), images::Column::Path)),
            "product_image",
        )
        .filter(condition);

    select = match query.sort {
        ProductsSortOrder::DescriptionAsc => {
            select.order_by(products::Column::Description, Order::Asc)
        }
        ProductsSortOrder::DescriptionDesc => {
            select.order_by(products::Column::Description, Order::Desc)
        }
        ProductsSortOrder::CodeAsc => select.order_by(products::Column::Code, Order::Asc),
        ProductsSortOrder::CodeDesc => select.order_by(products::Column::Code, Order::Desc),
        ProductsSortOrder::BlockedAsc => select.order_by(products::Column::Blocked, Order::Asc),
        ProductsSortOrder::BlockedDesc => select.order_by(products::Column::Blocked, Order::Desc),
        ProductsSortOrder::TypeAsc => select.order_by(types::Column::Name, Order::Asc),
        ProductsSortOrder::TypeDesc => select.order_by(types::Column::Name, Order::Desc),
        ProductsSortOrder::ClassAsc => select.order_by(classes::Column::Name, Order::Asc),
        ProductsSortOrder::ClassDesc => select.order_by(classes::Column::Name, Order::Desc),
        ProductsSortOrder::BrandAsc => select.order_by(brands::Column::Name, Order::Asc),
        ProductsSortOrder::BrandDesc => select.order_by(brands::Column::Name, Order::Desc),
    };

    let paginator = select
        .into_model::<GetProductsDTO>()
        .paginate(db.get_ref(), page_size);

    let total_pages = match paginator.num_pages().await {
        Ok(n) => n,
        Err(err) => {
            warn!("(get_products) Could not count products: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when counting products"
            }));
        }
    };

    match paginator.fetch_page(page).await {
        Ok(found_products) => HttpResponse::Ok().json(json!({
            "data": found_products,
            "page": page,
            "page_size": page_size,
            "total_pages": total_pages,
        })),
        Err(err) => {
            warn!("(get_products) Could not get products data: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving products data"
            }))
        }
    }
}
