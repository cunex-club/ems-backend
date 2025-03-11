use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::candidate::Candidate,
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
    models::traits::TopLevelGetById as _,
    permissions::DefaultAuthorizer,
    prelude::*,
    query::QueryablePlaceholder,
};
use uuid::Uuid;

#[get("/{id}")]
pub async fn get_candidate_details(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(_user): LoggedIn,
    candidate_id: Path<Uuid>,
    RequestType {
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<(), QueryablePlaceholder, SortablePlaceholder>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let candidate_id = candidate_id.into_inner();

    let candidate = Candidate::get_by_id(
        pool,
        candidate_id,
        fetch_level,
        descendant_fetch_level,
        &DefaultAuthorizer,
    )
    .await?;
    let response = ResponseType::new(candidate, None);

    Ok(HttpResponse::Ok().json(response))
}
