use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{election::db::DbElection, question::Question, Authorize},
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
struct CreateQuestionRequest {
    pub election_id: Uuid,
    pub question: MultiLangString,
    pub select_amount: i32,
    pub faculty_code: String,
    pub student_year_start: i32,
    pub student_year_end: i32,
    pub student_program: String,
}

#[post("")]
pub async fn create_question(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<CreateQuestionRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool: &sqlx::Pool<sqlx::Postgres> = &data.db;
    let Some(question) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            "/questions".to_string(),
        ));
    };

    // Check Election Exists
    let election = DbElection::get_by_id(pool, question.election_id).await?;

    // Check if User is authorized to create question
    election
        .authorize(user.id, pool, ActionType::Update)
        .await?;

    let mut transaction = pool.begin().await?;

    // Create Election

    let created_question_id = query!(
        r#"
        INSERT INTO questions (election_id, question_th, question_en, select_amount, faculty_code, student_year_start, student_year_end, student_program)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
        question.election_id,
        question.question.th,
        question.question.en,
        question.select_amount,
        question.faculty_code,
        question.student_year_start,
        question.student_year_end,
        question.student_program
    )
    .fetch_one(&mut *transaction)
    .await?
    .id;

    transaction.commit().await?;

    let question = Question::get_by_id(
        pool,
        created_question_id,
        fetch_level,
        descendant_fetch_level,
        &DefaultAuthorizer,
    )
    .await?;
    Ok(HttpResponse::Created().json(ResponseType::new(question, None)))
}
