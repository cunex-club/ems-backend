use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{candidate::db::DbCandidate, question::Question, Authorize},
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
pub async fn delete_candidate(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    candidate_id: Path<Uuid>,
    _: RequestType<(), QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let storage_client = &data.storage_client;
    let candidate_id = candidate_id.into_inner();

    // Check if election exists
    let candidate = DbCandidate::get_by_id(pool, candidate_id).await?;

    // Check if user has permission to delete election
    candidate
        .authorize(user.id, pool, ActionType::Delete)
        .await?;

    // Delete election
    query!("DELETE FROM candidates WHERE id = $1", candidate_id)
        .execute(pool)
        .await?;

    let filename = candidate.image_file.split('/').next_back().unwrap_or("");

    if !filename.is_empty() {
        storage_client
            .object()
            .delete("ems-candidate-profile", filename)
            .await
            .map_err(|err| {
                Error::InternalServerError(err.to_string(), format!("v1/candidates/{candidate_id}"))
            })?;
    }

    let response: ResponseType<Option<Question>> = ResponseType::new(None, None);

    Ok(HttpResponse::Accepted().json(response))
}
