use super::common::I64String;

use sea_orm::ActiveEnum;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct GetOrdersReq {
    pub team_id: I64String,
    pub page_index: usize,
    pub page_size: usize,
}

#[derive(Serialize)]
pub struct GetOrdersResp {
    /// 前端组件需要 total 总数量除以 page_size 计算总页数
    pub total: usize,
    pub orders: Vec<Order>,
}

#[derive(Serialize)]
pub struct Order {
    order_id: I64String,
    // TODO add a new type for NaiveDateTime for yyyy-mm-dd hh:mm:ss format?
    created_at: String,
    r#type: String,
    product_id: I64String,
    status: String,
    price: i32,
    service: String,
}

impl Order {
    pub fn from_db_model(order: crate::db_schema_codegen::orders::Model) -> Self {
        Self {
            order_id: order.id.into(),
            created_at: order.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            r#type: order.r#type.to_value(),
            product_id: order.product_id.into(),
            status: order.status.to_value(),
            price: order.price,
            // service_id: order.service_id.into(),
            service: "todo".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateOrderReq {
    pub team_id: I64String,
    pub user_id: I64String,
    #[serde(rename = "type")]
    pub type_: String,
    pub product_id: I64String,
    // TODO
}

#[derive(Serialize, Debug)]
pub struct CreateOrderResp {
    pub order_id: I64String,
}
