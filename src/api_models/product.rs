use chrono::Duration;

use super::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    product_id: I64String,
    name: String,
    created_time: String,
    r#type: String,
    price: f32,
    month_price: f32,
    specification: String,
}

impl Product {
    pub fn from_db_model(product: db_schema::data::products::Model) -> Self {
        Self {
            product_id: product.id.into(),
            name: product.name,
            created_time: product.created_at.to_string(),
            r#type: product.r#type,
            price: product.price.unwrap_or(0) as f32 / 100.0,
            month_price: (product.month_price.unwrap_or(0) * today_to_next_month_hours(None))
                as f32
                / 100.0,
            specification: product.specification,
        }
    }
}

// api: /api/v1/idp-shop/product/list
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetProductsReq {
    pub page_index: usize,
    pub page_size: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProductsRes {
    pub page_index: usize,
    pub page_size: usize,
    pub page_items: usize,
    pub total_pages: usize,
    pub total_items: usize,
    pub products: Vec<Product>,
}

// api: /api/v1/idp-shop/product/detail
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetProductDetailReq {
    pub product_id: I64String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProductDetailRes {
    pub product_id: I64String,
    pub name: String,
    pub created_at: String,
    pub r#type: String,
    pub price: f32,
    pub month_price: f32,
    pub specification: String,
    pub detail: String,
}

// api: /api/v1/idp-shop/product/price
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub product_id: I64String,
    pub r#type: String, 
    pub which_price: String,
    pub duration: I64String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetProductPriceReq {
    pub products: Vec<Price>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProductPriceRes {
    pub price: f32,
}
