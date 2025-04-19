use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{
        project::{db::DbProject, Project},
        user::User,
    },
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json, Path},
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
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct AddMemberRequest {
    member_id: Uuid,
}

#[post("/{id}/members")]
pub async fn add_member(
    data: Data<AppState>,
    _: ApiKeyHeader,
    project_id: Path<Uuid>,
    LoggedIn(user): LoggedIn,
    Json(RequestType {
        data: request_data, ..
    }): Json<RequestType<AddMemberRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let project_id = project_id.into_inner();

    let Some(project) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/projects/{project_id}"),
        ));
    };

    // Check if the project exists
    let db_project = DbProject::get_by_id(pool, project_id).await?;

    // Check if the user is the owner of the project
    if db_project.owner_id != user.id {
        return Err(Error::InvalidPermission(
            "User is not the owner of the project".to_string(),
            format!("/projects/{project_id}/members"),
        ));
    }

    // Check if the member exists
    let member = User::get_by_id(pool, project.member_id).await?;

    // Add the member to the project
    query!(
        r#"
        INSERT INTO project_members (project_id, user_id)
        SELECT $1, $2
        WHERE NOT EXISTS (
            SELECT 1 FROM project_members WHERE project_id = $1 AND user_id = $2
        )
        "#,
        project_id,
        member.id
    )
    .execute(pool)
    .await?;

    let response: ResponseType<Option<Project>> = ResponseType::new(None, None);

    Ok(HttpResponse::Created().json(response))
}
