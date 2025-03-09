use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{
        question::{db::DbQuestion, Question},
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
struct ModifyQuestionRequest {
    pub question: Option<FlexibleMultiLangString>,
    pub faculty_code: Option<String>,
    pub student_year_start: Option<i32>,
    pub student_year_end: Option<i32>,
    pub student_program: Option<String>,
}

#[put("/{id}")]
pub async fn modify_question(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    question_id: Path<Uuid>,
    Json(RequestType {
        data: request_data,
        fetch_level,
        descendant_fetch_level,
        ..
    }): Json<RequestType<ModifyQuestionRequest, QueryablePlaceholder, SortablePlaceholder>>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let question_id = question_id.into_inner();

    let Some(election) = request_data else {
        return Err(Error::InvalidRequest(
            "Json deserialize error: field `data` can not be empty".to_string(),
            format!("/questions/{question_id}"),
        ));
    };

    // Check if the project exists
    let db_question = DbQuestion::get_by_id(pool, question_id).await?;

    // Check if the user is the owner of the project
    db_question
        .authorize(user.id, pool, ActionType::Update)
        .await?;

    let mut transaction = pool.begin().await?;

    let mut qb = SqlSetClause::new();

    qb.push_update_field("faculty_code", election.faculty_code, QueryParam::String)
        .push_multilang_update_field("question", election.question)
        .push_update_field(
            "student_year_start",
            election.student_year_start.map(i64::from),
            QueryParam::Int,
        )
        .push_update_field(
            "student_year_end",
            election.student_year_end.map(i64::from),
            QueryParam::Int,
        )
        .push_update_field(
            "student_program",
            election.student_program,
            QueryParam::String,
        );
    let mut qb = qb.into_query_builder("UPDATE questions");
    qb.push(" WHERE id = ")
        .push_bind(question_id)
        .build()
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    let question = Question::get_by_id(
        pool,
        question_id,
        fetch_level,
        descendant_fetch_level,
        &DefaultAuthorizer,
    )
    .await?;

    Ok(HttpResponse::Ok().json(ResponseType::new(question, None)))
}
