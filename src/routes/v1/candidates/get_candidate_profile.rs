use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::candidate::db::DbCandidate,
    AppState,
};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use mysk_lib::{models::traits::GetById, prelude::*};
use uuid::Uuid;

#[get("/{id}/profile")]
pub async fn get_candidate_profile(
    data: Data<AppState>,
    _: ApiKeyHeader,
    LoggedIn(_user): LoggedIn,
    candidate_id: Path<Uuid>,
) -> Result<impl Responder> {
    let pool = &data.db;
    let storage_client = &data.storage_client;
    let candidate_id = candidate_id.into_inner();

    // Check if the candidate exists
    let db_candidate = DbCandidate::get_by_id(pool, candidate_id).await?;

    let filename = db_candidate.image_file.split('/').next_back().unwrap_or("");

    // Get the image to the storage
    let image = match storage_client
        .object()
        .read("ems-candidate-profile", filename)
        .await
    {
        Ok(created_image) => created_image,
        Err(err) => {
            return Err(Error::InternalServerError(
                err.to_string(),
                format!("v1/candidates/{candidate_id}/upload"),
            ))
        }
    };

    let bytes = storage_client
        .object()
        .download("ems-candidate-profile", filename)
        .await
        .map_err(|err| {
            Error::InternalServerError(
                err.to_string(),
                format!("v1/candidates/{candidate_id}/upload"),
            )
        })?;

    let response = HttpResponse::Ok()
        .content_type(image.content_type.unwrap_or("image/jpeg".to_string()))
        .body(bytes);

    Ok(response)
}
