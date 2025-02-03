use crate::models::project::db::DbProject;
use async_trait::async_trait;
use mysk_lib::{
    common::requests::FetchLevel, models::traits::FetchLevelVariant, permissions::Authorizer,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactProject {
    pub id: Uuid,
    pub name: String,
    pub election_count: i64,
    pub member_count: i64,
}

#[async_trait]
impl FetchLevelVariant<DbProject> for CompactProject {
    async fn from_table(
        pool: &PgPool,
        table: DbProject,
        _descendant_fetch_level: Option<&FetchLevel>,
        _authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        Ok(Self {
            id: table.id,
            name: table.name,
            election_count: DbProject::get_election_count(pool, table.id).await?,
            member_count: DbProject::get_member_count(pool, table.id).await?,
        })
    }
}
