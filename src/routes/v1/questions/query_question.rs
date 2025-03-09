use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::question::{
        requests::{queryable::QueryableQuestion, sortable::SortableQuestion},
        Question,
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
pub async fn query_question(
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
    }: RequestType<(), QueryableQuestion, SortableQuestion>,
) -> Result<impl Responder> {
    let pool = &data.db;

    let (questions, pagination) = Question::query(
        pool,
        fetch_level,
        descendant_fetch_level,
        filter,
        sort,
        pagination,
        &DefaultAuthorizer,
    )
    .await?;
    let response = ResponseType::new(questions, Some(MetadataType::new(Some(pagination))));

    Ok(HttpResponse::Ok().json(response))
}
