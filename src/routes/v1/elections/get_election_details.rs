use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{
        election::{db::DbElection, Election},
        Authorize,
    },
    AppState,
};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{
    common::{
        requests::{RequestType, SortablePlaceholder},
        response::ResponseType,
    },
    models::traits::{GetById, TopLevelGetById as _},
    permissions::ActionType,
    permissions::DefaultAuthorizer,
    prelude::*,
    query::QueryablePlaceholder,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn get_election_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    election_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<(), QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let election_id = election_id.into_inner();

    // Check if the user has permission to read the election
    let db_election = DbElection::get_by_id(pool, election_id).await?;
    db_election
        .authorize(user.id, pool, ActionType::ReadIdOnly)
        .await?;

    let election = Election::get_by_id(
        pool,
        election_id,
        fetch_level,
        descendant_fetch_level,
        &DefaultAuthorizer,
    )
    .await?;
    let response = ResponseType::new(election, None);

    Ok(HttpResponse::Ok().json(response))
}
