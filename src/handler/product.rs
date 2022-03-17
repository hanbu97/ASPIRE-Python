use std::vec;

use crate::api_models::today_to_next_month_hours;

use super::*;
use api_models::product::*;

use db_schema::{data::products, sea_orm::Condition};
use migration::sea_query::Expr;

// get products list
pub async fn get_products(
    conn: &DatabaseConnection,
    req: &GetProductsReq,
) -> anyhow::Result<(usize, usize, usize, Vec<Product>)> {
    let paginator = Products::find()
        .filter(products::Column::IsDeleted.eq(false))
        .order_by_desc(products::Column::CreatedAt)
        .paginate(conn, req.page_size);
    let total_items = paginator.num_items().await?;
    let products = paginator.fetch_page(req.page_index).await?;

    let total_pages = (total_items as f64 / req.page_size as f64).ceil() as usize;

    Ok((
        products.len(),
        total_items,
        total_pages,
        products.into_iter().map(Product::from_db_model).collect(),
    ))
}

// get product detail by product_id
pub async fn get_detail(
    conn: &DatabaseConnection,
    req: &GetProductDetailReq,
) -> anyhow::Result<(String, String, String, String, String, f32, f32, i64)> {
    let product: Option<products::Model> = Products::find()
        .filter(products::Column::IsDeleted.eq(false))
        .filter(products::Column::Id.eq(req.product_id.to_i64()))
        .one(conn)
        .await?;

    match product {
        Some(product) => Ok((
            product.detail.unwrap_or("".to_string()),
            product.name,
            product.specification,
            product.created_at.to_string(),
            product.r#type,
            product.price.unwrap_or(0) as f32 / 100.0,
            (product.month_price.unwrap_or(0) * today_to_next_month_hours(None)) as f32 / 100.0,
            product.id,
        )),
        None => Err(anyhow!("Product not found")),
    }
}

// post a list of product with product_id, type, which_price, duration to get total price
// api: /api/v1/idp-shop/product/price
pub async fn calculate_products_price(
    conn: &DatabaseConnection,
    req: &GetProductPriceReq,
) -> anyhow::Result<f32> {
    let product_ids: Vec<i64> = req.products.iter().map(|r| r.product_id.to_i64()).collect();

    let items: Vec<products::Model> = Products::find()
        .filter(products::Column::Id.is_in(product_ids))
        .all(conn)
        .await?;

    if items.len() != req.products.len() {
        return Err(anyhow!("Some product not found"));
    }

    let mut p = 0i32;
    for (item, r) in items.iter().zip(&req.products) {
        let tmp_p = match r.which_price.as_str() {
            "price" => item.price.unwrap_or(0) * r.duration.to_i64() as i32,
            "month_price" => {
                item.month_price.unwrap_or(0)
                    * today_to_next_month_hours(Some(r.duration.to_i64() as i32))
            }
            _ => 0,
        };
        p += tmp_p;
    }

    Ok(p as f32 / 100.0)
}
