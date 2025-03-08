use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{
        election::{db::DbElection, Election},
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
pub async fn delete_election(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    election_id: Path<Uuid>,
    _: RequestType<(), QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let election_id = election_id.into_inner();

    // Check if election exists
    let election = DbElection::get_by_id(pool, election_id).await?;

    // Check if user has permission to delete election
    election
        .authorize(user.id, pool, ActionType::Delete)
        .await?;

    // Delete election
    query!("DELETE FROM elections WHERE id = $1", election_id)
        .execute(pool)
        .await?;

    let response: ResponseType<Option<Election>> = ResponseType::new(None, None);

    Ok(HttpResponse::Accepted().json(response))
}
