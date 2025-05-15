use crate::{extractors::api_key::ApiKeyHeader, models::project::db::DbProject, AppState};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{models::traits::GetById, prelude::*};
use uuid::Uuid;

#[get("/{id}/export")]
pub async fn export_project(
    data: Data<AppState>,
    _: ApiKeyHeader,
    // LoggedIn(_user): LoggedIn,
    project_id: Path<Uuid>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let storage_client = &data.storage_client;
    let project_id = project_id.into_inner();

    let project = DbProject::get_by_id(pool, project_id).await?;

    let bytes = project.export(pool, storage_client).await?;

    // respond with zip file
    let filename = format!("{}.zip", project.name);
    let response = HttpResponse::Ok()
        .content_type("application/zip")
        .append_header(actix_web::http::header::ContentDisposition {
            disposition: actix_web::http::header::DispositionType::Attachment,
            parameters: vec![actix_web::http::header::DispositionParam::Filename(
                filename,
            )],
        })
        .body(bytes);
    Ok(response)
}
