use crate::models::{election::db::DbElection, project::Project, question::Question};
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
pub struct DefaultElection {
    pub id: Uuid,
    pub project: Project,
    pub label: String,
    pub name: MultiLangString,
    pub questions: Vec<Question>,
    pub header: MultiLangString,
    pub detail: Option<MultiLangString>,
}

#[async_trait]
impl FetchLevelVariant<DbElection> for DefaultElection {
    async fn from_table(
        pool: &PgPool,
        table: DbElection,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        let question_ids = DbElection::get_questions(pool, table.id).await?;

        Ok(Self {
            id: table.id,
            project: Project::get_by_id(
                pool,
                table.project_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            label: table.label,
            name: MultiLangString {
                th: table.name_th,
                en: Some(table.name_en),
            },
            questions: Question::get_by_ids(
                pool,
                question_ids,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            header: MultiLangString {
                th: table.header_th,
                en: Some(table.header_en),
            },
            detail: table.detail_th.map(|th| MultiLangString {
                th,
                en: table.detail_en,
            }),
        })
    }
}
