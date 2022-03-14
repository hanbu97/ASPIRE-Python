use crate::handler;
use axum::routing::{get, on, post, MethodFilter};
use axum::Router;

pub fn init_router() -> Router {
    Router::new().nest(
        "/api/v1/idp-shop",
        Router::new()
            .nest(
                "/order",
                Router::new()
                    .route("/list", get(handler::order::list_orders))
                    .route("/", post(handler::order::create_order)),
            )
            .nest(
                "/finance",
                Router::new().route(
                    "/account",
                    on(MethodFilter::GET, handler::finance::get_finance_account),
                ),
            ), // .nest("/demo", get(get_demos).post(create_demo)),
    )
}
