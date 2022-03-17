use super::*;

#[derive(Serialize)]
pub struct Service {
    service_id: I64String,
    created_at: DateTime,
    updated_at: DateTime,
    start_time: DateTime,
    expire_time: DateTime,
    product_id: I64String,
    status: String,
}

impl Service {
    pub fn from_db_model(service: db_schema::data::services::Model) -> Self {
        Self {
            service_id: service.id.into(),
            created_at: service.created_at,
            updated_at: service.updated_at,
            start_time: service.start_time,
            expire_time: service.expire_time,
            product_id: service.product_id.into(),
            status: service.status.into(),
        }
    }
}

// api: /api/v1/idp-shop/service/list
#[derive(Deserialize, Debug)]
pub struct GetServicesReq {
    pub team_id: usize,
    pub page_index: usize,
    pub page_size: usize,
}

#[derive(Serialize)]
pub struct GetServicesRes {
    pub page_index: usize,
    pub total_pages: usize,
    pub services: Vec<Service>,
}
