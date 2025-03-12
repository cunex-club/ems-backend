use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::candidate::{
        requests::{queryable::QueryableCandidate, sortable::SortableCandidate},
        Candidate,
    },
    AppState,
};
use actix_web::{get, web::Data, HttpResponse, Responder};
use mysk_lib::{
    common::{
        requests::RequestType,
        response::{MetadataType, ResponseType},
    },
    models::traits::TopLevelQuery,
    permissions::DefaultAuthorizer,
    prelude::*,
};

#[get("")]
pub async fn query_candidates(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(_user): LoggedIn,
    RequestType {
        pagination,
        filter,
        sort,
        fetch_level,
        descendant_fetch_level,
        ..
    }: RequestType<(), QueryableCandidate, SortableCandidate>,
) -> Result<impl Responder> {
    let pool = &data.db;

    let (candidates, pagination) = Candidate::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &DefaultAuthorizer,
    )
    .await?;
    let response = ResponseType::new(candidates, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
