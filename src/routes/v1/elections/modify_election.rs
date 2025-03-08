use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{
        election::{db::DbElection, Election},
        Authorize,
    },
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
        string::FlexibleMultiLangString,
    },
    models::traits::{GetById, TopLevelGetById},
    permissions::{ActionType, DefaultAuthorizer},
    prelude::*,
    query::{QueryParam, QueryablePlaceholder, SqlSetClause},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ModifyProjectRequest {
    label: Option<String>,
    name: Option<FlexibleMultiLangString>,
    header: Option<FlexibleMultiLangString>,
    details: Option<FlexibleMultiLangString>,
}

#[put("/{id}")]
pub async fn modify_election(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    election_id: Path<Uuid>,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<ModifyProjectRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let election_id = election_id.into_inner();

    let Some(election) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/elections/{election_id}"),
        ));
    };

    // Check if the project exists
    let db_election = DbElection::get_by_id(pool, election_id).await?;

    // Check if the user is the owner of the project
    db_election
        .authorize(user.id, pool, ActionType::Update)
        .await?;

    let mut transaction = pool.begin().await?;

    let mut qb = SqlSetClause::new();

    qb.push_update_field("label", election.label, QueryParam::String)
        .push_multilang_update_field("name", election.name)
        .push_multilang_update_field("header", election.header)
        .push_multilang_update_field("details", election.details);
    let mut qb = qb.into_query_builder("UPDATE elections");
    qb.push(" WHERE id = ")
        .push_bind(election_id)
        .build()
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    let election = Election::get_by_id(
        pool,
        election_id,
        fetch_level,
        descendant_fetch_level,
        &DefaultAuthorizer,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ResponseType::new(election, None)))
}
