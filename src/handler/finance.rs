use super::*;
use api_models::finance::*;

use db_schema::data::charge_record as charges;
use db_schema::data::finance_account as accounts;

pub async fn get_account_balance(
    conn: &DatabaseConnection,
    req: GetFinanceAccountReq,
) -> anyhow::Result<(f32, f32, f32)> {
    let account: Option<accounts::Model> = Accounts::find()
        .filter(accounts::Column::Id.eq(req.team_id.to_i64()))
        .one(conn)
        .await?;

    let (total_balance, blocked_balance) = account.map_or((0.0, 0.0), |a| {
        (
            a.total_balance as f32 / 100.0,
            a.blocked_balance as f32 / 100.0,
        )
    });
    let avl_balance = total_balance - blocked_balance;

    Ok((total_balance, avl_balance, blocked_balance))
}



// get charge status
// api: /api/v1/idp-shop/finace/charge/status
pub async fn get_finace_charge_status(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<GetChargeStatusReq>,
) -> Res<GetChargeStatusRes> {
    let charge: Option<charges::Model> = Charges::find()
        .filter(charges::Column::TeamId.eq(req.team_id.to_i64()))
        .filter(charges::Column::Id.eq(req.charge_id.to_i64()))
        .one(conn)
        .await
        .unwrap();

    if let Some(o) = charge {
        Res::success(GetChargeStatusRes {
            status: "success".into(),
        })
    } else {
        Res::fail()
    }
}
