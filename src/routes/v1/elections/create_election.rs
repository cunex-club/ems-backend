use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{election::Election, project::db::DbProject, Authorize},
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
        string::MultiLangString,
    },
    models::traits::{GetById, TopLevelGetById},
    permissions::{ActionType, DefaultAuthorizer},
    prelude::*,
    query::QueryablePlaceholder,
};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CreateElectionRequest {
    pub label: String,
    pub name: MultiLangString,
    pub header: MultiLangString,
    pub details: Option<MultiLangString>,
    pub project_id: Uuid,
}

#[post("")]
pub async fn create_election(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<CreateElectionRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let Some(election) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            "/elections".to_string(),
        ));
    };

    // Check Project Exists
    let project = DbProject::get_by_id(pool, election.project_id).await?;

    // Check if User is authorized to create election
    project.authorize(user.id, pool, ActionType::Update).await?;

    let mut transaction = pool.begin().await?;

    // Create Election

    let created_election_id = query!(
        r#"
        INSERT INTO elections (project_id, label, name_th, name_en, header_th, header_en, detail_th, detail_en)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
        election.project_id,
        election.label,
        election.name.th,
        election.name.en,
        election.header.th,
        election.header.en,
        match &election.details {
            Some(details) => Some(details.th.clone()),
            None => None,
        },
        match &election.details {
            Some(details) => details.en.clone(),
            None => None,
            
        }
    )
    .fetch_one(&mut *transaction)
    .await?
    .id;


    transaction.commit().await?;

    let election = Election::get_by_id(
        pool,
        created_election_id,
        fetch_level,
        descendant_fetch_level,
        &DefaultAuthorizer,
    )
    .await?;
    Ok(HttpResponse::Created().json(ResponseType::new(election, None)))
}
