use chrono::{DateTime, Utc};
use mysk_lib::prelude::*;
use mysk_lib_macros::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT * FROM elections",
    count_query = "SELECT COUNT(*) FROM elections"
)]
pub struct DbElection {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub project_id: Uuid,
    pub label: String,
    pub name_th: String,
    pub name_en: String,
    pub header_th: String,
    pub header_en: String,
    pub detail_th: Option<String>,
    pub detail_en: Option<String>,
}

impl DbElection {
    pub async fn get_questions(pool: &sqlx::PgPool, election_id: Uuid) -> Result<Vec<Uuid>> {
        Ok(sqlx::query!(
            r#"
            SELECT id
            FROM questions
            WHERE election_id = $1
            "#,
            election_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.id)
        .collect())
    }

    pub async fn get_question_count(pool: &sqlx::PgPool, election_id: Uuid) -> Result<i64> {
        Ok(sqlx::query!(
            r#"
            SELECT COUNT(*)
            FROM questions
            WHERE election_id = $1
            "#,
            election_id
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0))
    }
}
