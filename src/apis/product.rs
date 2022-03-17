use super::*;

use crate::{
    api_models::product::*,
    handler::product::{get_detail, get_products},
};

// get products list
// api: /api/v1/idp-shop/product/list
pub async fn get_product_list(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<GetProductsReq>,
    cookies: Cookies,
) -> core::result::Result<Res<GetProductsRes>, Res<String>> {
    match get_products(&conn, &req).await {
        Ok(res) => Ok(Res::success(GetProductsRes {
            page_index: req.page_index,
            page_size: req.page_size,
            total_items: res.1,
            page_items: res.0,
            total_pages: res.2,
            products: res.3,
        })),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}

// get product detail by product_id
// api: /api/v1/idp-shop/product/detail
pub async fn get_product_detail(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<GetProductDetailReq>,
    cookies: Cookies,
) -> core::result::Result<Res<GetProductDetailRes>, Res<String>> {
    match get_detail(&conn, &req).await {
        Ok(res) => Ok(Res::success(GetProductDetailRes {
            detail: res.0,
            name: res.1,
            specification: res.2,
            created_at: res.3,
            r#type: res.4,
            price: res.5,
            month_price: res.6,
            product_id: res.7.into(),
        })),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
