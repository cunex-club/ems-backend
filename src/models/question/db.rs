use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib::{
    common::requests::FilterConfig, models::traits::QueryDb, permissions::ActionType, prelude::*,
    query::Queryable,
};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::Authorize;

use super::requests::{queryable::QueryableQuestion, sortable::SortableQuestion};

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
    pub select_amount: i32,
    pub question_en: String,
    pub faculty_code: String,
    pub student_year_start: i32,
    pub student_year_end: i32,
    pub student_program: String,
    pub question_order: i32,
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

    pub async fn get_canon_id(pool: &sqlx::PgPool, id: Uuid) -> Result<String> {
        // canon id is EXX-XX-XX where EXX is the election id and XX is the question id
        // SELECT election_id, question_order, choice_order FROM candidates WHERE id = $1 INNER JOIN questions ON questions.id = candidates.question_id

        let question = sqlx::query!(
            r#"
                SELECT election_id, question_order
                FROM questions
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        match question {
            Some(question) => {
                let election_id = question.election_id.simple().to_string();
                let question_order = question.question_order;
                Ok(format!("{election_id}-{question_order}"))
            }
            None => Err(Error::EntityNotFound(
                "Question".to_string(),
                format!("questions/{id}"),
            )),
        }
    }
}

#[async_trait]
impl Authorize for DbQuestion {
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
                WHERE id = (SELECT project_id FROM elections where id = $1) AND owner_id = $2
                "#,
            self.election_id,
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
                WHERE project_id = (SELECT project_id FROM elections where id = $1) AND user_id = $2
                "#,
            self.election_id,
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
                "Question Authorizer".to_string(),
                "User is not authorized to create question".to_string(),
            ))
        }
    }
}

#[async_trait]
impl QueryDb<QueryableQuestion, SortableQuestion> for DbQuestion {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableQuestion>>,
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
