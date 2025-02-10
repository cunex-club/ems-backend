// use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT * FROM candidates",
    count_query = "SELECT COUNT(*) FROM candidates"
)]
pub struct DbCandidate {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub question_id: Uuid,
    pub choice_label_th: String,
    pub choice_label_en: String,
    pub title: String,
    pub info_line_1: String,
    pub info_line_2: String,
    pub info_line_3: String,
    pub info_line_4: String,
    pub info_line_5: String,
    pub body_title_1: String,
    pub body_1: String,
    pub body_title_2: String,
    pub body_2: String,
    pub image_file: String,
}
