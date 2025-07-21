use async_trait::async_trait;
// use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mysk_lib::{
    common::requests::FilterConfig, models::traits::QueryDb, permissions::ActionType, prelude::*,
    query::Queryable,
};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::{election::db::DbElection, question::db::DbQuestion, Authorize};

use super::requests::{queryable::QueryableCandidate, sortable::SortableCandidate};

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
    pub choice_order: i32,
    pub uploaded_at: Option<DateTime<Utc>>,
}

impl DbCandidate {
    pub async fn get_canon_id(pool: &sqlx::PgPool, id: Uuid) -> Result<String> {
        // canon id is EXX-XX-XX where EXX is the election id and XX is the question id and XX is the candidate id each from order
        // SELECT election_id, question_order, choice_order FROM candidates WHERE id = $1 INNER JOIN questions ON questions.id = candidates.question_id

        let candidate = sqlx::query!(
            r#"
                SELECT election_id, question_order, choice_order
                FROM candidates
                INNER JOIN questions ON questions.id = candidates.question_id
                WHERE candidates.id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        match candidate {
            Some(candidate) => {
                let election_id = DbElection::get_canon_id(pool, candidate.election_id).await?;
                let question_order = candidate.question_order;
                let choice_order = candidate.choice_order;

                let canon_id = format!("{election_id}-{question_order:02}-{choice_order:02}");
                Ok(canon_id)
            }
            None => Err(Error::EntityNotFound(
                "Candidate not found".to_string(),
                format!("candidates/{id}"),
            )),
        }
    }
    pub async fn get_canon_candidateinfo_insert(
        pool: &sqlx::PgPool,
        id: Vec<Uuid>,
    ) -> Result<String> {
        // Return string of tuple for sql insert
        // (`ID`, `ElectionID`, `Title`, `InfoLine1`, `InfoLine2`, `InfoLine3`, `InfoLine4`, `InfoLine5`, `InfoTitle1`, `infoBody1`, `InfoTitle2`, `infoBody2`, `ImageFile`)

        let candidates = sqlx::query!(
            r#"
                SELECT candidates.id, election_id, title, info_line_1, info_line_2, info_line_3, info_line_4, info_line_5, body_title_1, body_1, body_title_2, body_2, image_file
                FROM candidates
                INNER JOIN questions on questions.id = candidates.question_id
                WHERE candidates.id = ANY($1)
            "#,
            &id
        )
        .fetch_all(pool)
        .await?;

        let mut result = "INSERT INTO `candidateinfo` (`ID`, `ElectionID`, `Title`, `InfoLine1`, `InfoLine2`, `InfoLine3`, `InfoLine4`, `InfoLine5`, `InfoTitle1`, `infoBody1`, `InfoTitle2`, `infoBody2`, `ImageFile`) VALUES ".to_string();
        for candidate in candidates {
            let id = DbCandidate::get_canon_id(pool, candidate.id).await?;
            let election_id = DbElection::get_canon_id(pool, candidate.election_id).await?;
            let title = candidate.title;
            let info_line_1 = candidate.info_line_1;
            let info_line_2 = candidate.info_line_2;
            let info_line_3 = candidate.info_line_3;
            let info_line_4 = candidate.info_line_4;
            let info_line_5 = candidate.info_line_5;
            let body_title_1 = candidate.body_title_1;
            let body_1 = candidate.body_1.escape_debug().to_string();
            let body_title_2 = candidate.body_title_2;
            let body_2 = candidate.body_2.escape_debug().to_string();
            let image_file_extension = candidate.image_file.split('.').next_back().unwrap_or("");
            let image_file = id.clone() + "." + image_file_extension;

            result.push_str(&format!(
                "\n('{id}', '{election_id}', '{title}', '{info_line_1}', '{info_line_2}', '{info_line_3}', '{info_line_4}', '{info_line_5}', '{body_title_1}', '{body_1}', '{body_title_2}', '{body_2}', '{image_file}'),"
            ));
        }

        // Remove the last comma
        if result.ends_with(',') {
            result.pop();
        }
        result.push(';');
        // Return the result
        Ok(result)
    }

    pub async fn get_canon_choicemapping_insert(
        pool: &sqlx::PgPool,
        id: Vec<Uuid>,
    ) -> Result<String> {
        // Return string of tuple for sql insert
        // (`QuestionID`, `ChoiceID`, `ChoiceTH`, `ChoiceEN`)
        let candidates = sqlx::query!(
            r#"
                SELECT candidates.id, question_id, choice_order, choice_label_th, choice_label_en
                FROM candidates
                WHERE candidates.id = ANY($1)
            "#,
            &id
        )
        .fetch_all(pool)
        .await?;

        let mut result = "INSERT INTO `choicemapping` (`ID`, `QuestionID`, `ChoiceID`, `ChoiceTH`, `ChoiceEN`) VALUES ".to_string();
        for candidate in candidates {
            let id = DbCandidate::get_canon_id(pool, candidate.id).await?;
            let question_id = DbQuestion::get_canon_id(pool, candidate.question_id).await?;
            let choice_order = candidate.choice_order;
            let choice_label_th = candidate.choice_label_th;
            let choice_label_en = candidate.choice_label_en;

            result.push_str(&format!(
                "\n('{id}', '{question_id}', '{choice_order}', '{choice_label_th}', '{choice_label_en}'),"
            ));
        }

        // Remove the last comma
        if result.ends_with(',') {
            result.pop();
        }
        result.push(';');
        // Return the result
        Ok(result)
    }
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

#[async_trait]
impl QueryDb<QueryableCandidate, SortableCandidate> for DbCandidate {
    fn build_shared_query(
        query_builder: &mut QueryBuilder<'_, Postgres>,
        filter: Option<FilterConfig<QueryableCandidate>>,
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
