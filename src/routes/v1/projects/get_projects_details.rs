use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::project::Project,
    AppState,
};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::traits::TopLevelGetById as _,
    permissions::DefaultAuthorizer,
    prelude::*,
    query::QueryablePlaceholder,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn get_projects_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(_user): LoggedIn,
    project_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<(), QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let project_id = project_id.into_inner();

    let project = Project::get_by_id(
        pool,
        project_id,
        fetch_level.as_ref(),
        descendant_fetch_level.as_ref(),
        &DefaultAuthorizer,
    )
    .await?;
    let response = ResponseType::new(project, None);

    Ok(HttpResponse::Ok().json(response))
}
