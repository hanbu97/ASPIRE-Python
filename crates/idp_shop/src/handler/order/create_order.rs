use crate::api_models::common::Resp;
use crate::api_models::order::{CreateOrderReq, CreateOrderResp};
use crate::app_context::AppContext;

use axum::extract::Extension;
use axum::extract::Query;
use sea_orm::{EntityTrait, Set};

use crate::db_schema_codegen::orders;

pub async fn create_order(
    Extension(app_context): Extension<AppContext>,
    Query(req): Query<CreateOrderReq>,
) -> Resp<CreateOrderResp> {
    let team_id = req.team_id.to_i64();
    let new_order = orders::ActiveModel {
        team_id: Set(team_id),
        user_id: Set(1),
        ..Default::default()
    };
    let b = orders::Entity::insert(new_order)
        .exec(&app_context.db)
        .await
        .unwrap();
    dbg!(b);
    todo!()
}

#[tokio::test]
async fn test_create_order() {
    let app_context = AppContext::new_for_mock_in_test().await;

    create_order(
        app_context,
        Query(CreateOrderReq {
            team_id: 1.into(),
            user_id: 1.into(),
            type_: "purchase".to_string(),
            product_id: 1.into(),
        }),
    )
    .await;
}
