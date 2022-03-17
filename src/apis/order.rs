use super::*;

use crate::{api_models::order::*, handler::order::get_orders};

// get order list by team_id
// api: /api/v1/idp-shop/order/list
pub async fn get_order_list(
    Extension(ref conn): Extension<DatabaseConnection>,
    cookies: Cookies,
    Query(req): Query<GetOrdersReq>,
) -> core::result::Result<Res<GetOrdersRes>, Res<String>> {
    let cookie_ids = match get_ids_from_cookie(&cookies) {
        Ok(ids) => ids,
        Err(e) => {
            return Err(Res::custom_fail(
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            ));
        }
    };

    match get_orders(&conn, cookie_ids.team_id, &req).await {
        Ok(res) => Ok(Res::success(GetOrdersRes {
            page_index: req.page_index,
            page_size: req.page_size,
            total_items: res.1,
            page_items: res.0,
            total_pages: res.2,
            orders: res.3,
        })),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
