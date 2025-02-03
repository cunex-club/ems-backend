use crate::models::election::db::DbElection;
use async_trait::async_trait;
use mysk_lib::{
    common::{requests::FetchLevel, string::MultiLangString},
    models::traits::FetchLevelVariant,
    permissions::Authorizer,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactElection {
    pub id: Uuid,
    pub label: String,
    pub name: MultiLangString,
    pub question_count: i64,
}

#[async_trait]
impl FetchLevelVariant<DbElection> for CompactElection {
    async fn from_table(
        pool: &PgPool,
        table: DbElection,
        _descendant_fetch_level: Option<&FetchLevel>,
        _authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        Ok(Self {
            id: table.id,
            label: table.label,
            name: MultiLangString {
                th: table.name_th,
                en: Some(table.name_en),
            },
            question_count: DbElection::get_question_count(pool, table.id).await?,
        })
    }
}
