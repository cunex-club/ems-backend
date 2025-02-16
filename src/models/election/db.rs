use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib::{permissions::ActionType, prelude::*};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::Authorize;

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

#[async_trait]
impl Authorize for DbElection {
    async fn authorize(
        &self,
        user_id: Uuid,
        pool: &sqlx::PgPool,
        action: ActionType,
    ) -> Result<()> {
        match action {
            _ => {
                // If user is owner of project or is member of project
                let is_owner = sqlx::query!(
                    r#"
                    SELECT COUNT(*)
                    FROM projects
                    WHERE id = $1 AND owner_id = $2
                    "#,
                    self.project_id,
                    user_id
                )
                .fetch_one(pool)
                .await?
                .count
                .unwrap_or(0)
                    > 0;

                let is_member = sqlx::query!(
                    r#"
                    SELECT COUNT(*)
                    FROM project_members
                    WHERE project_id = $1 AND user_id = $2
                    "#,
                    self.project_id,
                    user_id
                )
                .fetch_one(pool)
                .await?
                .count
                .unwrap_or(0)
                    > 0;

                if is_owner || is_member {
                    Ok(())
                } else {
                    Err(Error::InvalidPermission(
                        "Election Authorizer".to_string(),
                        "User is not authorized to create election".to_string(),
                    ))
                }
            }
        }
    }
}
