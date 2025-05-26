#![allow(clippy::too_many_lines)]

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
use sqlx::query;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ModifyQuestionRequest {
    pub question: Option<FlexibleMultiLangString>,
    pub select_amount: Option<i32>,
    pub faculty_code: Option<String>,
    pub student_year_start: Option<i32>,
    pub student_year_end: Option<i32>,
    pub student_program: Option<String>,
    pub new_question_order: Option<i32>,
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
            "select_amount",
            election.select_amount.map(i64::from),
            QueryParam::Int,
        )
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

    if qb.0.len() != 1 {
        let mut qb = qb.into_query_builder("UPDATE questions");
        qb.push(" WHERE id = ")
            .push_bind(question_id)
            .build()
            .execute(&mut *transaction)
            .await?;
    }

    if let Some(new_question_order) = election.new_question_order {
        // Check if the new question order is valid
        let max_question_order: i32 = query!(
            r#"
            SELECT COALESCE(MAX(question_order), 0) as res FROM questions WHERE election_id = $1
            "#,
            db_question.election_id
        )
        .fetch_one(&mut *transaction)
        .await?
        .res
        .unwrap_or(0);

        // If the new question order is greater than the current maximum, return an error
        if new_question_order > max_question_order && new_question_order > 0 {
            return Err(Error::InvalidRequest(
                "New question order is greater than the current maximum question order and greater than 0"
                    .to_string(),
                format!("/questions/{question_id}"),
            ));
        }

        // Get current question_order
        let current_question_order = query!(
            r#"
            SELECT question_order FROM questions WHERE id = $1
            "#,
            question_id
        )
        .fetch_one(&mut *transaction)
        .await?
        .question_order;

        // Update question_order for all questions in the same election by
        // incrementing or decrementing their question_order
        match current_question_order.cmp(&new_question_order) {
            std::cmp::Ordering::Greater => {
                query!(
                    r#"
                    UPDATE questions
                    SET question_order = question_order + 1
                    WHERE election_id = (SELECT election_id FROM questions WHERE id = $1)
                    AND question_order >= $2 AND question_order < $3
                    "#,
                    question_id,
                    new_question_order,
                    current_question_order
                )
                .execute(&mut *transaction)
                .await?;
            }
            std::cmp::Ordering::Less => {
                query!(
                    r#"
                    UPDATE questions
                    SET question_order = question_order - 1
                    WHERE election_id = (SELECT election_id FROM questions WHERE id = $1)
                    AND question_order > $2 AND question_order <= $3
                    "#,
                    question_id,
                    current_question_order,
                    new_question_order
                )
                .execute(&mut *transaction)
                .await?;
            }
            std::cmp::Ordering::Equal => {}
        }

        // Update question_order for the current question
        query!(
            r#"
            UPDATE questions
            SET question_order = $2
            WHERE id = $1
            "#,
            question_id,
            new_question_order
        )
        .execute(&mut *transaction)
        .await?;
    }

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
