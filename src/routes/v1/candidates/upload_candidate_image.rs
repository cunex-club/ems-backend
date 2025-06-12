use crate::{
    extractors::{api_key::ApiKeyHeader, logged_in::LoggedIn},
    models::{
        candidate::{db::DbCandidate, Candidate},
        Authorize,
    },
    AppState,
};
use actix_web::{
    post,
    web::{Bytes, Data, Path},
    HttpRequest, HttpResponse, Responder,
};
use image::ImageReader;
use mysk_lib::{
    common::response::ResponseType, models::traits::GetById, permissions::ActionType, prelude::*,
};
use sqlx::query;
use std::io::Cursor;
use uuid::Uuid;

#[post("/{id}/upload")]
pub async fn upload_candidate_image(
    data: Data<AppState>,
    candidate_id: Path<Uuid>,
    _: ApiKeyHeader,
    LoggedIn(user): LoggedIn,
    request: HttpRequest,
    image: Bytes,
) -> Result<impl Responder> {
    let pool = &data.db;
    let storage_client = &data.storage_client;
    let candidate_id = candidate_id.into_inner();
    let image = image.to_vec();

    let content_type = match request.headers().get("Content-Type") {
        Some(content_type) => content_type.to_str().unwrap_or(""),
        None => {
            return Err(Error::InvalidRequest(
                "Invalid Content-Type".to_string(),
                format!("v1/candidates/{candidate_id}/upload"),
            ))
        }
    };

    // Check if the content type is valid
    if !content_type.starts_with("image/") {
        return Err(Error::InvalidRequest(
            "Invalid Content-Type".to_string(),
            format!("v1/candidates/{candidate_id}/upload"),
        ));
    }

    let image = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .map_err(|_| {
            Error::InvalidRequest(
                "Invalid image format".to_string(),
                format!("v1/candidates/{candidate_id}/upload"),
            )
        })?
        .decode()
        .map_err(|_| {
            Error::InvalidRequest(
                "Invalid image data".to_string(),
                format!("v1/candidates/{candidate_id}/upload"),
            )
        })?;

    // if the image is not square return an error
    if image.width() != image.height() {
        return Err(Error::InvalidRequest(
            "Image must be square".to_string(),
            format!("v1/candidates/{candidate_id}/upload"),
        ));
    }

    let image = image.resize(300, 300, image::imageops::FilterType::Lanczos3);

    // encode as jpg
    let mut image_buf = Cursor::new(Vec::new());
    image
        .write_to(&mut image_buf, image::ImageFormat::Jpeg)
        .map_err(|_| {
            Error::InternalServerError(
                "Failed to write image".to_string(),
                format!("v1/candidates/{candidate_id}/upload"),
            )
        })?;
    let image = image_buf.into_inner();

    // Check if the candidate exists
    let db_candidate = DbCandidate::get_by_id(pool, candidate_id).await?;

    db_candidate
        .authorize(user.id, pool, ActionType::Update)
        .await?;

    // Upload the image to the storage
    let created_image = match storage_client
        .object()
        .create(
            "ems-candidate-profile",
            image,
            &format!("{candidate_id}.jpg"),
            "image/jpeg",
        )
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

    query!(
        r#"
        UPDATE candidates
        SET image_file = $1, 
            uploaded_at = NOW()
        WHERE id = $2
        "#,
        created_image.name,
        candidate_id
    )
    .execute(pool)
    .await?;

    let response: ResponseType<Option<Candidate>> = ResponseType::new(None, None);

    Ok(HttpResponse::Created().json(response))
}
