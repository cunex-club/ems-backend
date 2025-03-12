use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{
        candidate::{db::DbCandidate, Candidate},
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
struct ModifyCandidateRequest {
    pub choice_label: Option<FlexibleMultiLangString>,
    pub title: Option<String>,
    pub info_line_1: Option<String>,
    pub info_line_2: Option<String>,
    pub info_line_3: Option<String>,
    pub info_line_4: Option<String>,
    pub info_line_5: Option<String>,
    pub body_title_1: Option<String>,
    pub body_1: Option<String>,
    pub body_title_2: Option<String>,
    pub body_2: Option<String>,
    pub image_file: Option<String>,
}

#[put("/{id}")]
pub async fn modify_candidate(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    candidate_id: Path<Uuid>,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<ModifyCandidateRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let candidate_id = candidate_id.into_inner();

    let Some(candidate) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/candidates/{candidate_id}"),
        ));
    };

    // Check if the project exists
    let db_candidate = DbCandidate::get_by_id(pool, candidate_id).await?;

    // Check if the user is the owner of the project
    db_candidate
        .authorize(user.id, pool, ActionType::Update)
        .await?;

    let mut transaction = pool.begin().await?;

    let mut qb = SqlSetClause::new();

    qb.push_multilang_update_field("choice_label", candidate.choice_label)
        .push_update_field("title", candidate.title, QueryParam::String)
        .push_update_field("info_line_1", candidate.info_line_1, QueryParam::String)
        .push_update_field("info_line_2", candidate.info_line_2, QueryParam::String)
        .push_update_field("info_line_3", candidate.info_line_3, QueryParam::String)
        .push_update_field("info_line_4", candidate.info_line_4, QueryParam::String)
        .push_update_field("info_line_5", candidate.info_line_5, QueryParam::String)
        .push_update_field("body_title_1", candidate.body_title_1, QueryParam::String)
        .push_update_field("body_1", candidate.body_1, QueryParam::String)
        .push_update_field("body_title_2", candidate.body_title_2, QueryParam::String)
        .push_update_field("body_2", candidate.body_2, QueryParam::String)
        .push_update_field("image_file", candidate.image_file, QueryParam::String);

    let mut qb = qb.into_query_builder("UPDATE candidates");
    qb.push(" WHERE id = ")
        .push_bind(candidate_id)
        .build()
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    let candidate = Candidate::get_by_id(
        pool,
        candidate_id,
        fetch_level,
        descendant_fetch_level,
        &DefaultAuthorizer,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ResponseType::new(candidate, None)))
}
