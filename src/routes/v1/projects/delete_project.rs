use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::project::{db::DbProject, Project},
    AppState,
};
use actix_web::{
    delete,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::traits::GetById,
    prelude::*,
    query::QueryablePlaceholder,
};
use sqlx::query;
use uuid::Uuid;

#[delete("/{id}")]
pub async fn delete_project(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    project_id: Path<Uuid>,
    _: RequestType<(), QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let project_id = project_id.into_inner();

    // Check if project exists
    DbProject::get_by_id(pool, project_id).await?;

    // Check if user has permission to delete project
    if !query!(
        r#"SELECT EXISTS (
            SELECT 1
            FROM projects
            WHERE id = $1 AND owner_id = $2
        ) AS exists"#,
        project_id,
        user.id
    )
    .fetch_one(pool)
    .await?
    .exists
    .unwrap_or(false)
    {
        return Err(Error::InvalidPermission(
            format!("/projects/{project_id}"),
            "You do not have permission to delete this project".to_string(),
        ));
    };

    // Delete project
    query!("DELETE FROM projects WHERE id = $1", project_id)
        .execute(pool)
        .await?;

    let response: ResponseType<Option<Project>> = ResponseType::new(None, None);

    Ok(HttpResponse::Accepted().json(response))
}
