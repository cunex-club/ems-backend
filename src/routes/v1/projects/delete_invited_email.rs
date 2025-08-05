use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::project::{db::DbProject, Project},
    AppState,
};
use actix_web::{
    delete,
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
struct DeleteInvitedEmailRequest {
    email: String,
}

#[delete("/{id}/emails")]
pub async fn delete_invited_email(
    data: Data<AppState>,
    _: ApiKeyHeader,
    project_id: Path<Uuid>,
    LoggedIn(user): LoggedIn,
    Json(RequestType {
        data: request_data, ..
    }): Json<RequestType<DeleteInvitedEmailRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let project_id = project_id.into_inner();

    let Some(email) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/projects/{project_id}/emails"),
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

    // Check if the member is in the project
    let member_in_project = query!(
        r#"
        SELECT * FROM project_member_queue
        WHERE project_id = $1 AND email = $2
        "#,
        project_id,
        email.email
    )
    .fetch_optional(pool)
    .await?
    .is_some();
    if !member_in_project {
        return Err(Error::InvalidRequest(
            "Email is not in the project".to_string(),
            format!("/projects/{project_id}/emails"),
        ));
    }
    // Remove the member from the project
    query!(
        r#"
        DELETE FROM project_member_queue
        WHERE project_id = $1 AND email = $2
        "#,
        project_id,
        email.email
    )
    .execute(pool)
    .await?;

    let response: ResponseType<Option<Project>> = ResponseType::new(None, None);

    Ok(HttpResponse::Created().json(response))
}
