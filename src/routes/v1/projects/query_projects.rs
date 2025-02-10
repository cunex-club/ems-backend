use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::project::{
        requests::{queryable::QueryableProject, sortable::SortableProject},
        Project,
    },
    AppState,
};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::{
        requests::RequestType,
        response::{MetadataType, ResponseType},
    },
    models::traits::TopLevelQuery as _,
    permissions::DefaultAuthorizer,
    prelude::*,
};

#[get("")]
pub async fn query_projects(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(_user): LoggedIn,
    RequestType {
        pagination,
        filter,
        sort,
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<(), QueryableProject, SortableProject>,
) -> Result<impl Responder> {
    let pool = &data.db;
    // let authorizer = permissions::get_authorizer(pool, &user, "/students".to_string()).await?;

    let (project, pagination) = Project::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &DefaultAuthorizer,
    )
    .await?;
    let response = ResponseType::new(project, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
