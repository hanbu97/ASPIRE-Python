use crate::api_models::common::Resp;
use crate::api_models::order::{GetOrdersReq, GetOrdersResp};
use crate::app_context::AppContext;
use axum::extract::{Extension, Query};

///
/// @author Jin Huang
/// @date 2022/3/12 9:38 AM
///
pub async fn get_finance_account(
    Extension(app_context): Extension<AppContext>,
    req: Query<GetOrdersReq>,
) -> Resp<GetOrdersResp> {
    dbg!(req);
    // let orders = sqlx::query!(r#"select 1 as field_name"#)
    //     .fetch_one(&app_context.sqlx_db)
    //     .await
    //     .unwrap();
    // Resp::success(GetOrdersResp(Vec::new()))
    todo!()
}
