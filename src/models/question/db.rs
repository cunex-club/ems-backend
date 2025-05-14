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
                Ok(format!("{election_id}-{question_order:02}"))
            }
            None => Err(Error::EntityNotFound(
                "Question".to_string(),
                format!("questions/{id}"),
            )),
        }
    }

    // CREATE TABLE `questionlogic` (
    //     `ElectionID` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    //     `QuestionID` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    //     `QuestionTH` varchar(250) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    //     `QuestionEN` varchar(250) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    //     `QuestionType` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    //     `FacultyCode` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    //     `StudentYear_Start` int NOT NULL,
    //     `StudentYear_End` int NOT NULL,
    //     `StudentProgram` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    //     `Dormitory` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    //     `DayNight` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL
    //   ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
    pub async fn get_canon_questionlogic_insert(
        pool: &sqlx::PgPool,
        id: Vec<Uuid>,
    ) -> Result<String> {
        let question = sqlx::query!(
            r#"
                SELECT questions.id, election_id, question_order, question_th, question_en, faculty_code, student_year_start, student_year_end, student_program, select_amount, COUNT(candidates.id) as candidate_count
                FROM questions
                LEFT JOIN candidates ON questions.id = candidates.question_id
                WHERE questions.id = ANY($1)
                GROUP BY questions.id, election_id, question_order, question_th, question_en, faculty_code, student_year_start, student_year_end, student_program, select_amount
            "#,
            &id
        )
        .fetch_all(pool)
        .await?;

        let mut result = "INSERT INTO `questionlogic` (`ElectionID`, `QuestionID`, `QuestionTH`, `QuestionEN`, `QuestionType`, `FacultyCode`, `StudentYear_Start`, `StudentYear_End`, `StudentProgram`, `Dormitory`, `DayNight`) VALUES ".to_string();
        for question in question {
            let election_id = question.election_id.simple().to_string();
            let question_order = question.question_order;
            let question_th = question.question_th;
            let question_en = question.question_en;
            let faculty_code = question.faculty_code;
            let student_year_start = question.student_year_start;
            let student_year_end = question.student_year_end;
            let student_program = question.student_program;
            let question_type = if question.select_amount > 1 {
                format!("SELECT{}", question.select_amount)
            } else {
                question
                    .candidate_count
                    .map_or_else(|| "0".to_string(), |count| count.to_string())
            };
            result.push_str(&format!(
                "('{election_id}', '{election_id}-{question_order:02}', '{question_th}', '{question_en}', '{question_type}', '{faculty_code}', {student_year_start}, {student_year_end}, '{student_program}', '', '', ''),",
            ));
        }
        // Remove the last comma
        if result.ends_with(',') {
            result.pop();
        }
        // Return the result
        Ok(result)
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
