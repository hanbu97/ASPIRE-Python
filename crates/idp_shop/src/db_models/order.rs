use rbatis::CRUDTable;
use serde::{Deserialize, Serialize};

#[derive(CRUDTable, Serialize, Deserialize, Debug)]
pub struct Orders {
    pub id: i64,
    pub version: i32,

    pub team_id: i64,
    pub product_id: i64,
    pub user_id: i64,
    /// 订单关联的已购服务 id
    pub service_id: i64,
    pub status: String,
    pub r#type: String,
    /// 支付成功时间
    pub paid_at: chrono::NaiveDateTime,
    /// 订单金额单位为分
    pub price: i32,
}

#[tokio::test]
async fn a() {
    use rbatis::crud::CRUD;
    crate::init_logger();
    let rb = rbatis::rbatis::Rbatis::new();
    rb.link("postgres://w:w@localhost/w").await.unwrap();

    let resp = rb
        .fetch_list_by_wrapper::<Orders>(rb.new_wrapper())
        .await
        .unwrap();

    dbg!(resp);
}

#[derive(Debug)]
struct Order {
    team_id: i64,
    product_id: i64,
    user_id: i64,
    /// 订单关联的已购服务 id
    service_id: i64,
    // status: OrderStatus,
    /// 支付成功时间
    paid_at: i64,
    /// 订单金额单位为分
    price: i32,

    id: i64,
    created_at: i64,
    updated_at: i64,
    version: i32,
    is_deleted: bool,
}
