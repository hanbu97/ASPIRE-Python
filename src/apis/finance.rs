use std::fmt::Result;

use axum::http::{Result as rst, StatusCode};

use crate::{
    api_models::{common::Res, finance::*},
    handler::finance::get_account_balance,
};

use super::*;

// get finance account balance by team_id
// api: /api/v1/idp-shop/finace/account
pub async fn get_finance_account(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<GetFinanceAccountReq>,
) -> core::result::Result<Res<GetFinanceAccountRes>, (StatusCode, String)> {
    match get_account_balance(&conn, req).await {
        Ok(res) => Ok(Res::success(GetFinanceAccountRes {
            total_balance: res.0,
            avl_balance: res.1,
            blocked_balance: res.2,
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
