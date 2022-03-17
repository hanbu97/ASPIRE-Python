use crate::api_models::today_to_next_month_hours;

use super::*;

use api_models::order::*;

use chrono::Utc;
use db_schema::data::orders;
use db_schema::data::prelude::*;
use db_schema::data::products;
use db_schema::data::sea_orm_active_enums::OrderType;

use db_schema::data::sea_orm_active_enums::OrderStatus;
use db_schema::sea_orm::Set;
use db_schema::sea_orm::Value;
use futures::future::join_all;
use futures::TryFutureExt;
use migration::sea_query::Expr;

pub async fn get_orders(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<GetOrdersReq>,
) -> Res<GetOrdersRes> {
    let paginator = Orders::find()
        .filter(orders::Column::TeamId.eq(req.team_id.to_i64()))
        .order_by_desc(orders::Column::CreatedAt)
        .paginate(conn, req.page_size);
    let total_items = paginator.num_items().await.unwrap();
    let orders = paginator.fetch_page(req.page_index).await.unwrap();

    Res::success(GetOrdersRes {
        page_index: req.page_index,
        page_size: req.page_size,
        page_items: orders.len(),
        total_items,
        total_pages: total_items / req.page_size + 1,
        orders: orders.into_iter().map(Order::from_db_model).collect(),
    })
}

// get order detail by order_id
// api: /api/v1/idp-shop/order/detail
pub async fn get_order_detail(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<GetOrderDetailReq>,
) -> Res<GetOrderDetailRes> {
    let order: Option<orders::Model> = Orders::find()
        .filter(orders::Column::TeamId.eq(req.team_id.to_i64()))
        .filter(orders::Column::OrderId.eq(req.order_id.to_i64()))
        .one(conn)
        .await
        .unwrap();

    match order {
        Some(order) => Res::success(GetOrderDetailRes {
            order: Order::from_db_model(order),
        }),
        None => Res::fail(),
    }
}

// get order payment status by order_id
// api: /api/v1/idp-shop/order/pay
pub async fn get_order_pay(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<PayOrderReq>,
) -> Res<PayOrderRes> {
    let order: Option<orders::Model> = Orders::find()
        .filter(orders::Column::TeamId.eq(req.team_id.to_i64()))
        .filter(orders::Column::OrderId.eq(req.order_id.to_i64()))
        .one(conn)
        .await
        .unwrap();

    if let Some(o) = order {
        // match o.status {
        //     OrderStatus::Paid => Res::success(PayOrderRes {
        //         status: "paid".into(),
        //     }),
        //     OrderStatus::NotPaid => Res::success(PayOrderRes {
        //         status: "unpaid".into(),
        //     }),
        //     OrderStatus::PaidTimeoutCancel => Res::success(PayOrderRes {
        //         status: "paid_timeout_cancel".into(),
        //     }),
        //     OrderStatus::UserCancel => Res::success(PayOrderRes {
        //         status: "user_cancel".into(),
        //     }),
        //     _ => Res::fail(),
        // }
        Res::success(PayOrderRes {
            status: o.status.into(),
        })
    } else {
        Res::fail()
    }
}

// post a list of product with product_id, type, which_price, duration to create order, return order_id, total_price
// api: /api/v1/idp-shop/order/create
pub async fn post_create_order(
    Extension(ref conn): Extension<DatabaseConnection>,
    req: Json<PostCreateOrderReq>,
) -> Res<PostCreateOrderRes> {
    let product_ids: Vec<i64> = req.products.iter().map(|r| r.product_id.to_i64()).collect();

    let items: Vec<products::Model> = Products::find()
        .filter(products::Column::Id.is_in(product_ids))
        .all(conn)
        .await
        .unwrap();

    if items.len() != req.products.len() {
        return Res::fail();
    }

    let mut total_price = 0i32;
    let mut total_month_price = 0i32;

    let mut price_list = Vec::new();

    for (item, r) in items.iter().zip(&req.products) {
        match r.which_price.as_str() {
            "price" => {
                let _tmp = item.price.unwrap_or(0) * r.duration.to_i64() as i32;
                price_list.push(_tmp);
                total_price += _tmp;
            }
            "month_price" => {
                let _tmp = item.month_price.unwrap_or(0)
                    * today_to_next_month_hours(Some(r.duration.to_i64() as i32));
                price_list.push(_tmp);
                total_month_price += _tmp;
            }
            _ => (),
        };
    }

    let orders = join_all(items.iter().zip(req.products.clone()).zip(price_list).map(
        |((m, r), p)| {
            let order = orders::ActiveModel {
                user_id: Set(req.user_id.to_i64()),
                status: Set("created".into()),
                team_id: Set(req.team_id.to_i64()),
                price: Set(p),
                which_price: Set(r.which_price.into()),
                product_id: Set(r.product_id.to_i64()),
                r#type: Set(m.r#type.clone()),
                created_at: Set(Utc::now().naive_utc()),
                updated_at: Set(Utc::now().naive_utc()),
                is_deleted: Set(false),
                version: Set(0),
                duration: Set(r.duration.to_i64()),
                ..Default::default()
            };
            order.insert(conn)
        },
    ))
    .await;

    let mut order_ids: Vec<i64> = Vec::new();
    let orders = orders
        .into_iter()
        .zip(items)
        .map(|(p, i)| {
            let od = p.unwrap();
            order_ids.push(od.service_id);
            SubOrder {
                price: od.price as f32 / 100.0,
                product_id: od.product_id.into(),
                name: i.name.into(),
                which_price: od.which_price,
                r#type: i.r#type.into(),
                specification: i.specification,
                detail: i.detail.unwrap_or("".to_string()),
            }
        })
        .collect();

    let order_id = order_ids.iter().min().unwrap().to_owned();

    Orders::update_many()
        .col_expr(
            orders::Column::OrderId,
            Expr::value(Value::BigInt(Some(order_id))),
        )
        .filter(orders::Column::ServiceId.is_in(order_ids))
        .exec(conn)
        .await
        .unwrap();

    Res::success(PostCreateOrderRes {
        order_id: order_id.into(),
        total_price: total_price as f32 / 100.0,
        total_month_price: total_month_price as f32 / 100.0,
        orders,
    })
}

// post order payment status by order_id
// api: /api/v1/idp-shop/order/pay
pub async fn post_order_pay(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<PayOrderReq>,
) -> Res<PayOrderRes> {
    let order: Option<orders::Model> = Orders::find()
        .filter(orders::Column::TeamId.eq(req.team_id.to_i64()))
        .filter(orders::Column::OrderId.eq(req.order_id.to_i64()))
        .one(conn)
        .await
        .unwrap();

    if let Some(o) = order {
        // match o.status {
        //     OrderStatus::Paid => Res::success(PayOrderRes {
        //         status: "paid".into(),
        //     }),
        //     OrderStatus::NotPaid => Res::success(PayOrderRes {
        //         status: "unpaid".into(),
        //     }),
        //     OrderStatus::PaidTimeoutCancel => Res::success(PayOrderRes {
        //         status: "paid_timeout_cancel".into(),
        //     }),
        //     OrderStatus::UserCancel => Res::success(PayOrderRes {
        //         status: "user_cancel".into(),
        //     }),
        //     _ => Res::fail(),
        // }
        Res::success(PayOrderRes {
            status: o.status.into(),
        })
    } else {
        Res::fail()
    }
}
