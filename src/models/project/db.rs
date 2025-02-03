use chrono::{DateTime, Utc};
use mysk_lib::prelude::*;
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::election::db::DbElection;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT * FROM projects",
    count_query = "SELECT COUNT(*) FROM projects"
)]
pub struct DbProject {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name: String,
    pub owner_id: Uuid,
}

impl DbProject {
    pub async fn get_elections(pool: &sqlx::PgPool, project_id: Uuid) -> Result<Vec<Uuid>> {
        Ok(sqlx::query!(
            r#"
            SELECT id
            FROM elections
            WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.id)
        .collect())
    }

    pub async fn get_election_count(pool: &sqlx::PgPool, project_id: Uuid) -> Result<i64> {
        Ok(sqlx::query!(
            r#"
            SELECT COUNT(*)
            FROM elections
            WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0))
    }

    pub async fn get_members(pool: &sqlx::PgPool, project_id: Uuid) -> Result<Vec<Uuid>> {
        Ok(sqlx::query!(
            r#"
            SELECT user_id
            FROM project_members
            WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.user_id)
        .collect())
    }

    pub async fn get_member_count(pool: &sqlx::PgPool, project_id: Uuid) -> Result<i64> {
        Ok(sqlx::query!(
            r#"
            SELECT COUNT(*)
            FROM project_members
            WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0))
    }
}
