use crate::apis;
use crate::handler;
use axum::routing::{get, on, post, MethodFilter, MethodRouter};
use axum::Router;

pub fn init_router() -> Router {
    Router::new().nest(
        "/api/v1/idp-shop",
        Router::new()
            .nest(
                "/order",
                Router::new()
                    .route("/list", get(handler::order::get_orders))
                    .route("/detail", get(handler::order::get_order_detail))
                    .route(
                        "/pay",
                        get(handler::order::get_order_pay).post(handler::order::post_order_pay),
                    )
                    .route("/create", post(handler::order::post_create_order)),
            )
            .nest(
                "/product",
                Router::new()
                    .route("/list", get(apis::product::get_product_list))
                    .route("/detail", get(apis::product::get_product_detail))
                    .route("/price", post(handler::product::post_products_price)),
            )
            .nest(
                "/service",
                Router::new().route("/list", get(handler::service::get_services)),
            )
            .nest(
                "/finance",
                Router::new()
                    .nest(
                        "/account",
                        Router::new().route("/", get(apis::finance::get_finance_account)),
                    )
                    .nest(
                        "/charge",
                        Router::new()
                            .route("/status", get(apis::finance::get_finace_charge_status)),
                    ),
            ),
        // .nest("/demo", get(get_demos).post(create_demo)),
    )
}
