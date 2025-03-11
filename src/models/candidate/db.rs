use async_trait::async_trait;
// use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib::{permissions::ActionType, prelude::*};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::Authorize;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseQuery, GetById)]
#[base_query(
    query = "SELECT * FROM candidates",
    count_query = "SELECT COUNT(*) FROM candidates"
)]
pub struct DbCandidate {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub question_id: Uuid,
    pub choice_label_th: String,
    pub choice_label_en: String,
    pub title: String,
    pub info_line_1: String,
    pub info_line_2: String,
    pub info_line_3: String,
    pub info_line_4: String,
    pub info_line_5: String,
    pub body_title_1: String,
    pub body_1: String,
    pub body_title_2: String,
    pub body_2: String,
    pub image_file: String,
}

#[async_trait]
impl Authorize for DbCandidate {
    async fn authorize(
        &self,
        user_id: Uuid,
        pool: &sqlx::PgPool,
        _action: ActionType,
    ) -> Result<()> {
        // If user is owner of project or is member of project
        let is_owner = sqlx::query!(
            r#"
                SELECT COUNT(*)
                FROM projects
                WHERE id = (SELECT project_id FROM elections where id = (SELECT election_id FROM questions WHERE id = $1)) AND owner_id = $2
                "#,
            self.question_id,
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
                WHERE project_id = (SELECT project_id FROM elections where id = (SELECT election_id FROM questions WHERE id = $1)) AND user_id = $2
                "#,
            self.question_id,
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
                "Candidate Authorizer".to_string(),
                "User is not authorized to read/create/update/delete candidate".to_string(),
            ))
        }
    }
}
