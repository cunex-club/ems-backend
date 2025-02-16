use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::project::Project,
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::traits::TopLevelGetById,
    permissions::DefaultAuthorizer,
    prelude::*,
    query::QueryablePlaceholder,
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CreateProjectRequest {
    name: String,
    member_ids: Vec<Uuid>,
}

#[post("")]
pub async fn create_project(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<CreateProjectRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let Some(project) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            "/projects".to_string(),
        ));
    };

    let mut transaction = pool.begin().await?;

    let project_id = query!(
        r#"
        INSERT INTO projects (name, owner_id)
        VALUES ($1, $2)
        RETURNING id
        "#,
        project.name,
        user.id
    )
    .fetch_one(&mut *transaction)
    .await?
    .id;

    for member_id in project.member_ids {
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

    Ok(HttpResponse::Created().json(ResponseType::new(project, None)))
}
