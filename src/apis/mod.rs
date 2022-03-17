use anyhow::anyhow;
pub use axum::extract::{Extension, Json, Query};
use axum::http::StatusCode;

use db_schema::data::prelude::*;
pub use db_schema::sea_orm;
pub use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use serde::Serialize;
use tower_cookies::{CookieManagerLayer, Cookies};

use crate::api_models::common::Res;

pub mod finance;
pub mod order;
pub mod product;
pub mod service;

#[derive(Serialize)]
pub struct CookieIds {
    pub user_id: i64,
    pub team_id: i64,
}

// parse cookie
pub fn get_ids_from_cookie(cookies: &Cookies) -> anyhow::Result<CookieIds> {
    let user_id = if let Some(t) = cookies.get("userId") {
        t.value().parse::<i64>()?
    } else {
        return Err(anyhow!("userId not found"));
    };
    let team_id = if let Some(t) = cookies.get("teamId") {
        t.value().parse::<i64>()?
    } else {
        return Err(anyhow!("teamId not found"));
    };

    Ok(CookieIds { user_id, team_id })
}
