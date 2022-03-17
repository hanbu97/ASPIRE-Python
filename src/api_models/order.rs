// use super::common::I64String;
// use serde::{Deserialize, Serialize};

use super::*;

use db_schema::data::sea_orm_active_enums::{
    OrderStatus::{NotPaid, PaidTimeoutCancel, UserCancel},
    OrderType::{Purchase, Renewal},
};

#[derive(Serialize)]
pub struct Order {
    order_id: I64String,
    created_at: DateTime,
    r#type: String,
    product_id: I64String,
    status: String,
    price: f32,
    service_id: I64String,
}

impl Order {
    pub fn from_db_model(order: db_schema::data::orders::Model) -> Self {
        // let order_type = match order.r#type {
        //     Purchase => "purchase",
        //     Renewal => "renewal",
        //     _ => "",
        // };
        // let order_status = match order.status {
        //     NotPaid => "not_paid",
        //     PaidTimeoutCancel => "paid_timeout_cancel",
        //     UserCancel => "user_cancel",
        //     _ => "",
        // };

        Self {
            order_id: order.order_id.unwrap_or(0).into(),
            created_at: order.created_at,
            r#type: order.r#type.into(),
            product_id: order.product_id.into(),
            status: order.status.into(),
            price: order.price as f32,
            service_id: order.service_id.into(),
        }
    }
}

// api: /api/v1/idp-shop/order/list
#[derive(Deserialize, Debug)]
pub struct GetOrdersReq {
    pub team_id: I64String,
    pub page_index: usize,
    pub page_size: usize,
}

#[derive(Serialize)]
pub struct GetOrdersRes {
    pub page_index: usize,
    pub page_size: usize,
    pub page_items: usize,
    pub total_pages: usize,
    pub total_items: usize,
    pub orders: Vec<Order>,
}

// api: /api/v1/idp-shop/order/detail
#[derive(Deserialize, Debug)]
pub struct GetOrderDetailReq {
    pub team_id: I64String,
    pub order_id: I64String,
}

#[derive(Serialize)]
pub struct GetOrderDetailRes {
    pub order: Order,
}

// api: /api/v1/idp-shop/order/pay
#[derive(Deserialize, Debug)]
pub struct PayOrderReq {
    pub team_id: I64String,
    pub order_id: I64String,
}
#[derive(Serialize)]
pub struct PayOrderRes {
    pub status: String,
}

// api: /api/v1/idp-shop/order/pay post
#[derive(Deserialize, Debug)]
pub struct PostPayOrderReq {
    pub team_id: I64String,
    pub user_id: I64String,
    pub order_id: I64String,
    pub r#type: String,
}
#[derive(Serialize)]
pub struct PostPayOrderRes {
    pub status: String,
}

// api: /api/v1/idp-shop/order/create
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price {
    pub product_id: I64String,
    pub which_price: String,
    pub duration: I64String,
}

#[derive(Deserialize, Debug)]
pub struct PostCreateOrderReq {
    pub team_id: I64String,
    pub user_id: I64String,
    pub products: Vec<Price>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubOrder {
    pub product_id: I64String,
    pub name: String,
    pub which_price: String,
    pub r#type: String,
    pub price: f32,
    pub specification: String,
    pub detail: String,
}

#[derive(Serialize)]
pub struct PostCreateOrderRes {
    pub order_id: I64String,
    pub total_price: f32,
    pub total_month_price: f32,
    pub orders: Vec<SubOrder>,
}
