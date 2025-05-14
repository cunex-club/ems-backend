use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{
        question::{db::DbQuestion, Question},
        Authorize,
    },
    AppState,
};
use actix_web::{
    delete,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::traits::GetById,
    permissions::ActionType,
    prelude::*,
    query::QueryablePlaceholder,
};
use sqlx::query;
use uuid::Uuid;

#[delete("/{id}")]
pub async fn delete_question(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    question_id: Path<Uuid>,
    _: RequestType<(), QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let question_id = question_id.into_inner();

    // Check if election exists
    let question = DbQuestion::get_by_id(pool, question_id).await?;

    // Check if user has permission to delete election
    question
        .authorize(user.id, pool, ActionType::Delete)
        .await?;

    // Delete election
    query!("DELETE FROM questions WHERE id = $1", question_id)
        .execute(pool)
        .await?;

    // Update the question order
    query!(
        "UPDATE questions SET question_order = question_order - 1 WHERE question_order > $1",
        question.question_order
    )
    .execute(pool)
    .await?;

    let response: ResponseType<Option<Question>> = ResponseType::new(None, None);

    Ok(HttpResponse::Accepted().json(response))
}
