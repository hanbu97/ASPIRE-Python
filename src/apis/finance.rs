use super::*;

use crate::{
    api_models::finance::*,
    handler::finance::{get_account_balance, get_charge_status},
};

// get finance account balance by team_id
// api: /api/v1/idp-shop/finace/account
pub async fn get_finance_account(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<GetFinanceAccountReq>,
    cookies: Cookies,
) -> core::result::Result<Res<GetFinanceAccountRes>, Res<String>> {
    let cookie_ids = match get_ids_from_cookie(&cookies) {
        Ok(ids) => ids,
        Err(e) => {
            return Err(Res::custom_fail(
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            ));
        }
    };

    match get_account_balance(&conn, cookie_ids.user_id, cookie_ids.team_id, req).await {
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

// get charge status
// api: /api/v1/idp-shop/finace/charge/status
pub async fn get_finace_charge_status(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<GetChargeStatusReq>,
    cookies: Cookies,
) -> core::result::Result<Res<GetChargeStatusRes>, Res<String>> {
    let cookie_ids = match get_ids_from_cookie(&cookies) {
        Ok(ids) => ids,
        Err(e) => {
            return Err(Res::custom_fail(
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            ));
        }
    };

    match get_charge_status(&conn, cookie_ids.user_id, cookie_ids.team_id, req).await {
        Ok(res) => Ok(Res::success(GetChargeStatusRes { status: res })),
        Err(e) => Err(Res::custom_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )),
    }
}
