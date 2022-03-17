use super::*;

use crate::{api_models::product::*, handler::product::get_products};

// get products list
// api: /api/v1/idp-shop/product/list
pub async fn get_product_list(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<GetProductsReq>,
    cookies: Cookies,
) -> Res<GetProductsRes> {
    match get_products(&conn, &req).await {
        Ok(res) => Res::success(GetProductsRes {
            page_index: req.page_index,
            page_size: req.page_size,
            total_items: res.1,
            page_items: res.0,
            total_pages: res.2,
            products: res.3,
        }),
        Err(e) => Res::custom_fail(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
