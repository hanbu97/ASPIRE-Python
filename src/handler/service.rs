use super::*;
use api_models::service::*;

use db_schema::data::services;

// get products
// api: /api/v1/idp-shop/service/list
pub async fn get_services(
    Extension(ref conn): Extension<DatabaseConnection>,
    Query(req): Query<GetServicesReq>,
) -> Res<GetServicesRes> {
    let paginator = Services::find()
        .filter(services::Column::IsDeleted.eq(false))
        .filter(services::Column::TeamId.eq(req.team_id as i64))
        .order_by_desc(services::Column::UpdatedAt)
        .paginate(conn, req.page_size);
    let total_pages = paginator.num_pages().await.unwrap();
    let services = paginator.fetch_page(req.page_index).await.unwrap();

    Res::success(GetServicesRes {
        page_index: req.page_index,
        total_pages,
        services: services.into_iter().map(Service::from_db_model).collect(),
    })
}
