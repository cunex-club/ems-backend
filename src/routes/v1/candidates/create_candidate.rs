use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{candidate::Candidate, question::db::DbQuestion, Authorize},
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
struct CreateCandidateRequest {
    pub question_id: Uuid,
    pub choice_label: MultiLangString,
    pub title: String,
    pub info_line_1: String,
    pub info_line_2: String,
    pub info_line_3: String,
    pub info_line_4: String,
    pub info_line_5: String,
    pub body_title_1: String,
    pub body_1: String,
    pub body_title_2: String,
    pub body_2: String,
    pub image_file: String,
}

#[post("")]
pub async fn create_candidate(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<CreateCandidateRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool: &sqlx::Pool<sqlx::Postgres> = &data.db;
    let Some(candidate) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            "/candidates".to_string(),
        ));
    };

    // Check Election Exists
    let question = DbQuestion::get_by_id(pool, candidate.question_id).await?;

    // Check if User is authorized to create question
    question
        .authorize(user.id, pool, ActionType::Update)
        .await?;

    let mut transaction = pool.begin().await?;

    // Create Election

    let created_candidate_id = query!(
        r#"
        INSERT INTO candidates (question_id, choice_label_th, choice_label_en, title, info_line_1, info_line_2, info_line_3, info_line_4, info_line_5, body_title_1, body_1, body_title_2, body_2, image_file)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        RETURNING id
        "#,
        candidate.question_id,
        candidate.choice_label.th,
        candidate.choice_label.en,
        candidate.title,
        candidate.info_line_1,
        candidate.info_line_2,
        candidate.info_line_3,
        candidate.info_line_4,
        candidate.info_line_5,
        candidate.body_title_1,
        candidate.body_1,
        candidate.body_title_2,
        candidate.body_2,
        candidate.image_file

    )
    .fetch_one(&mut *transaction)
    .await?
    .id;

    transaction.commit().await?;

    let candidate = Candidate::get_by_id(
        pool,
        created_candidate_id,
        fetch_level,
        descendant_fetch_level,
        &DefaultAuthorizer,
    )
    .await?;
    Ok(HttpResponse::Created().json(ResponseType::new(candidate, None)))
}
