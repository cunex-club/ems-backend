use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::common::PaginationType;

#[derive(Debug, Serialize)]
pub struct MetadataType {
    timestamp: DateTime<Utc>,
    pagination: Option<PaginationType>,
}

impl Default for MetadataType {
    fn default() -> Self {
        MetadataType {
            timestamp: Utc::now(),
            pagination: None,
        }
    }
}

impl MetadataType {
    pub fn new(pagination: Option<PaginationType>) -> Self {
        MetadataType {
            timestamp: Utc::now(),
            pagination,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResponseType<T> {
    api_version: String,
    data: Option<T>,
    error: Option<String>,
    meta: MetadataType,
}

impl<T> ResponseType<T> {
    pub fn new(data: T, meta: Option<MetadataType>) -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();

        ResponseType {
            api_version: version,
            data: Some(data),
            error: None,
            meta: meta.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EmptyResponseData;

#[derive(Debug, Serialize)]
pub struct ErrorType {
    pub id: Uuid,
    pub code: i64,
    pub error_type: String,
    pub detail: String,
    pub source: String,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ id: {}, code: {}, error_type: {}, detail: {}, source: {} }}",
            self.id, self.code, self.error_type, self.detail, self.source,
        )
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponseType {
    api_version: String,
    error: ErrorType,
    data: Option<String>,
    meta: Option<MetadataType>,
}

impl ErrorResponseType {
    pub fn new(error: ErrorType, meta: Option<MetadataType>) -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();

        ErrorResponseType {
            api_version: version,
            error,
            data: None,
            meta,
        }
    }
}

impl Display for ErrorResponseType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ api_version: {}, error: {}, data: {:?}, meta: {:?} }}",
            self.api_version, self.error, self.data, self.meta,
        )
    }
}
