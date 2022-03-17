use super::*;

#[derive(Serialize)]
pub struct Account {
    product_id: I64String,
    name: String,
    created_time: DateTime,
    r#type: String,
    price: f32,
    specification: String,
}

impl Account {
    pub fn from_db_model(product: db_schema::data::products::Model) -> Self {
        Self {
            product_id: product.id.into(),
            name: product.name,
            created_time: product.created_at,
            r#type: product.r#type,
            price: product.price.unwrap_or(0) as f32 / 100.0,
            specification: product.detail.unwrap_or("".to_string()),
        }
    }
}

// api: /api/v1/idp-shop/finance/account
#[derive(Deserialize, Debug)]
pub struct GetFinanceAccountReq {
    pub team_id: I64String,
}

#[derive(Serialize)]
pub struct GetFinanceAccountRes {
    pub total_balance: f32,
    pub avl_balance: f32,
    pub blocked_balance: f32,
}

// api: /api/v1/idp-shop/finance/charge/status
#[derive(Deserialize, Debug)]
pub struct GetChargeStatusReq {
    pub team_id: I64String,
    pub charge_id: I64String,
}
#[derive(Serialize)]
pub struct GetChargeStatusRes {
    pub status: String,
}
