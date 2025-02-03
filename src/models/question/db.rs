use chrono::{DateTime, Utc};
use mysk_lib::prelude::*;
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT * FROM questions",
    count_query = "SELECT COUNT(*) FROM questions"
)]
pub struct DbQuestion {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub election_id: Uuid,
    pub question_th: String,
    pub question_en: String,
    pub faculty_code: String,
    pub student_year_start: i64,
    pub student_year_end: i64,
    pub student_program: String,
}

impl DbQuestion {
    pub async fn get_candidates(pool: &sqlx::PgPool, question_id: Uuid) -> Result<Vec<Uuid>> {
        Ok(sqlx::query!(
            r#"
            SELECT id
            FROM candidates
            WHERE question_id = $1
            "#,
            question_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.id)
        .collect())
    }
}
