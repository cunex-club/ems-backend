use crate::models::{election::Election, project::db::DbProject, user::User};
use async_trait::async_trait;
use mysk_lib::{
    common::requests::FetchLevel,
    models::traits::{FetchLevelVariant, TopLevelGetById},
    permissions::Authorizer,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultProject {
    pub id: Uuid,
    pub name: String,
    pub elections: Vec<Election>,
    pub members: Vec<User>,
    pub owner: User,
}

#[async_trait]
impl FetchLevelVariant<DbProject> for DefaultProject {
    async fn from_table(
        pool: &PgPool,
        table: DbProject,
        descendant_fetch_level: Option<FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        let election_ids = DbProject::get_elections(pool, table.id).await?;
        let member_ids = DbProject::get_members(pool, table.id).await?;

        Ok(Self {
            id: table.id,
            name: table.name,
            elections: Election::get_by_ids(
                pool,
                election_ids,
                descendant_fetch_level,
                Some(FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            members: User::get_by_ids(pool, member_ids).await?,
            owner: User::get_by_id(pool, table.owner_id).await?,
        })
    }
}
