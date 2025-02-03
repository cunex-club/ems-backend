use crate::models::{candidate::Candidate, question::db::DbQuestion};
use async_trait::async_trait;
use mysk_lib::{
    common::{requests::FetchLevel, string::MultiLangString},
    models::traits::{FetchLevelVariant, TopLevelGetById},
    permissions::Authorizer,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultQuestion {
    pub id: Uuid,
    pub election_id: Uuid,
    pub question: MultiLangString,
    pub faculty_code: String,
    pub student_year_start: i64,
    pub student_year_end: i64,
    pub student_program: String,
    pub candidates: Vec<Candidate>,
}

#[async_trait]
impl FetchLevelVariant<DbQuestion> for DefaultQuestion {
    async fn from_table(
        pool: &PgPool,
        table: DbQuestion,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        let candidate_ids = DbQuestion::get_candidates(pool, table.id).await?;

        Ok(Self {
            id: table.id,
            election_id: table.election_id,
            question: MultiLangString {
                th: table.question_th,
                en: Some(table.question_en),
            },
            faculty_code: table.faculty_code,
            student_year_start: table.student_year_start,
            student_year_end: table.student_year_end,
            student_program: table.student_program,
            candidates: Candidate::get_by_ids(
                pool,
                candidate_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
        })
    }
}
