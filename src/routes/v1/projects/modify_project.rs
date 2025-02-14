use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::project::{db::DbProject, Project},
    AppState,
};
use actix_web::{
    put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::traits::{GetById, TopLevelGetById},
    permissions::DefaultAuthorizer,
    prelude::*,
    query::{QueryParam, QueryablePlaceholder, SqlSetClause},
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ModifyProjectRequest {
    name: Option<String>,
    owner_id: Option<Uuid>,
    member_ids: Option<Vec<Uuid>>,
}

#[put("/{id}")]
pub async fn modify_project(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    project_id: Path<Uuid>,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<ModifyProjectRequest, QueryablePlaceholder, SortablePlaceholder>>,
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
            "/projects".to_string(),
        ));
    }

    let mut transaction = pool.begin().await?;

    let mut qb = SqlSetClause::new();
    qb.push_update_field("name", project.name, QueryParam::String)
        .push_update_field("owner_id", project.owner_id, QueryParam::Uuid);
    let mut qb = qb.into_query_builder("UPDATE projects");
    qb.push(" WHERE id = ")
        .push_bind(project_id)
        .build()
        .execute(&mut *transaction)
        .await?;

    // Delete all members of the project
    query!(
        r#"
        DELETE FROM project_members
        WHERE project_id = $1
        "#,
        project_id
    )
    .execute(&mut *transaction)
    .await?;

    // Insert new members
    for member_id in project.member_ids.unwrap_or_default() {
        query!(
            r#"
            INSERT INTO project_members (project_id, user_id)
            VALUES ($1, $2)
            "#,
            project_id,
            member_id
        )
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;

    let project = Project::get_by_id(
        pool,
        project_id,
        fetch_level,
        descendant_fetch_level,
        &DefaultAuthorizer,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ResponseType::new(project, None)))
}
