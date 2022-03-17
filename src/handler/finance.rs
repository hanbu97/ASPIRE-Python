use super::*;
use api_models::finance::*;

use db_schema::data::charge_record as charges;
use db_schema::data::finance_account as accounts;

pub async fn get_account_balance(
    conn: &DatabaseConnection,
    user_id: i64,
    team_id: i64,
    req: GetFinanceAccountReq,
) -> anyhow::Result<(f32, f32, f32)> {
    let account: Option<accounts::Model> = Accounts::find()
        .filter(accounts::Column::IsDeleted.eq(false))
        .filter(accounts::Column::TeamId.eq(team_id))
        .filter(accounts::Column::Id.eq(req.account_id.to_i64()))
        .one(conn)
        .await?;

    if account.is_none() {
        return Err(anyhow!("Account not found"));
    }

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
pub async fn get_charge_status(
    conn: &DatabaseConnection,
    user_id: i64,
    team_id: i64,
    req: GetChargeStatusReq,
) -> anyhow::Result<String> {
    let charge: Option<charges::Model> = Charges::find()
        .filter(charges::Column::IsDeleted.eq(false))
        .filter(charges::Column::TeamId.eq(team_id))
        .filter(charges::Column::Id.eq(req.charge_id.to_i64()))
        .one(conn)
        .await?;

    if let Some(o) = charge {
        Ok(o.status.to_string())
    } else {
        Err(anyhow!("Charge not found"))
    }
}
