use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib::{
    common::requests::FilterConfig, models::traits::QueryDb, prelude::*, query::Queryable,
};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres, QueryBuilder};
use uuid::Uuid;

use super::requests::{queryable::QueryableProject, sortable::SortableProject};

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

#[async_trait]
impl QueryDb<QueryableProject, SortableProject> for DbProject {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableProject>>,
    ) {
        if let Some(filter) = filter {
            if let Some(data) = &filter.data {
                data.clone()
                    .to_where_clause()
                    .append_into_query_builder(query_builder);
            }
        }
    }
}
