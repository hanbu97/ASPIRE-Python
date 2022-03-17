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
    cookies: Cookies,
) -> core::result::Result<Res<GetFinanceAccountRes>, Res<String>> {
    let user_id = match cookies.get("userId") {
        Some(c) => c.value().parse::<i64>().unwrap(),
        None => {
            return Err(Res::custom_fail(
                StatusCode::INTERNAL_SERVER_ERROR,
                "userId not found in cookie".to_string(),
            ))
        }
    };

    let team_id = match cookies.get("teamId") {
        Some(c) => c.value().parse::<i64>().unwrap(),
        None => {
            return Err(Res::custom_fail(
                StatusCode::INTERNAL_SERVER_ERROR,
                "teamId not found in cookie".to_string(),
            ))
        }
    };

    match get_account_balance(&conn, user_id, team_id, req).await {
        Ok(res) => Ok(Res::success(GetFinanceAccountRes {
            total_balance: res.0,
            avl_balance: res.1,
            blocked_balance: res.2,
        })),

        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
