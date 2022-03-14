use crate::api_models::common::Resp;
use crate::api_models::order::{GetOrdersReq, GetOrdersResp, Order};
use crate::app_context::AppContext;
use crate::db_models::order::Orders;

use axum::extract::Extension;
use axum::extract::Query;
use rbatis::crud::CRUD;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::db_schema_codegen::orders;

pub async fn list_orders(
    Extension(app_context): Extension<AppContext>,
    Query(req): Query<GetOrdersReq>,
) -> Resp<GetOrdersResp> {
    // TODO 参数检查失败返回 400 错误码，别 panic
    debug_assert!(req.page_index >= 1);
    debug_assert!(req.page_size >= 1);

    // 前端的分页 page_index 从 1 开始但是数据库分页从 0 开始计数，这里需要转换一下
    let page_index = req.page_index - 1;

    let paginator = orders::Entity::find()
        .filter(orders::Column::TeamId.eq(req.team_id.to_i64()))
        .order_by_desc(orders::Column::CreatedAt)
        .paginate(&app_context.db, req.page_size);

    let total = paginator.num_items().await.unwrap();
    // let total_pages = paginator.num_pages().await.unwrap();
    let orders = paginator.fetch_page(page_index).await.unwrap();

    Resp::success(GetOrdersResp {
        total,
        orders: orders.into_iter().map(Order::from_db_model).collect(),
    })
}

/// http://localhost:9001/api/v1/idp-shop/order/list?team_id=1&page_index=1&page_size=1
#[deprecated]
pub async fn get_orders_rb(
    Extension(app_context): Extension<AppContext>,
    Query(req): Query<GetOrdersReq>,
) -> Resp<GetOrdersResp> {
    // TODO 参数检查失败返回 400 错误码，别 panic
    debug_assert!(req.page_index >= 1);
    debug_assert!(req.page_size >= 1);

    // 前端的分页 page_index 从 1 开始但是数据库分页从 0 开始计数，这里需要转换一下
    let page_index = req.page_index - 1;

    let page_req = rbatis::PageRequest::new(page_index as _, req.page_size as _);
    let rb = app_context.rb;
    let wrapper = rb
        .new_wrapper()
        .eq("team_id", req.team_id.to_i64())
        .order_by(false, &["created_at"]);
    let res = rb
        .fetch_page_by_wrapper::<Orders>(wrapper, &page_req)
        .await
        .unwrap();
    let total = res.total;
    let records = res.records;
    dbg!(records);

    Resp::success(GetOrdersResp {
        total: total as _,
        orders: Vec::new(),
    })
}
