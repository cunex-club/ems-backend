use std::io::{Cursor, Write};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cloud_storage::Client;
use mysk_lib::{
    common::requests::FilterConfig, models::traits::QueryDb, permissions::ActionType, prelude::*,
    query::Queryable,
};
use mysk_lib_macros::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;
use zip::{write::SimpleFileOptions, ZipWriter};

use crate::models::{
    candidate::db::DbCandidate, election::db::DbElection, question::db::DbQuestion, Authorize,
};

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
    pub async fn get_elections(pool: &PgPool, project_id: Uuid) -> Result<Vec<Uuid>> {
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

    pub async fn get_election_count(pool: &PgPool, project_id: Uuid) -> Result<i64> {
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

    pub async fn get_members(pool: &PgPool, project_id: Uuid) -> Result<Vec<Uuid>> {
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

    pub async fn get_invited_members(pool: &PgPool, project_id: Uuid) -> Result<Vec<String>> {
        Ok(sqlx::query!(
            r#"
            SELECT email
            FROM project_member_queue
            WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.email)
        .collect())
    }

    pub async fn get_member_count(pool: &PgPool, project_id: Uuid) -> Result<i64> {
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

    pub async fn export_canon_inserts(&self, pool: &PgPool) -> Result<String> {
        let mut result = String::new();

        let elections = Self::get_elections(pool, self.id).await?;

        let question_ids = query!(
            r#"
            SELECT id
            FROM questions
            WHERE election_id = ANY($1)
            "#,
            &elections
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.id)
        .collect::<Vec<_>>();

        let candidate_ids = query!(
            r#"
            SELECT id
            FROM candidates
            WHERE question_id = ANY($1)
            "#,
            &question_ids
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.id)
        .collect::<Vec<_>>();

        result.push_str(&DbElection::get_canon_electionmaster_insert(pool, elections).await?);

        result.push_str("\n\n\n");

        result.push_str(&DbQuestion::get_canon_questionlogic_insert(pool, question_ids).await?);

        result.push_str("\n\n\n");

        result.push_str(
            &DbCandidate::get_canon_choicemapping_insert(pool, candidate_ids.clone()).await?,
        );

        result.push_str("\n\n\n");

        result.push_str(&DbCandidate::get_canon_candidateinfo_insert(pool, candidate_ids).await?);
        Ok(result)
    }
    // Return byte array of zip file
    pub async fn export(&self, pool: &PgPool, storage_client: &Client) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        let mut zip = ZipWriter::new(Cursor::new(&mut buf));
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

        zip.start_file(
            format!("{}-{}-{}.sql", self.id, self.name, Utc::now().timestamp()),
            options,
        )
        .map_err(|e| Error::InternalServerError(e.to_string(), "DbProject::export".to_string()))?;

        zip.write(self.export_canon_inserts(pool).await?.as_bytes())
            .map_err(|e| {
                Error::InternalServerError(e.to_string(), "DbProject::export".to_string())
            })?;

        let candidates = query!(
                r#"
                SELECT id, image_file
                FROM candidates
                WHERE question_id = ANY(SELECT id FROM questions WHERE election_id = ANY(SELECT id FROM elections WHERE project_id = $1))
                "#,
                &self.id
            )
            .fetch_all(pool)
            .await?;

        zip.add_directory("images/", options).map_err(|e| {
            Error::InternalServerError(e.to_string(), "DbProject::export".to_string())
        })?;

        for candidate in candidates {
            if candidate.image_file.is_empty() {
                continue;
            }

            let extension = candidate.image_file.split('.').next_back().ok_or_else(|| {
                Error::InternalServerError(
                    "Invalid image file name".to_string(),
                    "DbProject::export".to_string(),
                )
            })?;

            let file_name = format!(
                "images/{}.{}",
                DbCandidate::get_canon_id(pool, candidate.id).await?,
                extension
            );
            zip.start_file(file_name, options).map_err(|e| {
                Error::InternalServerError(e.to_string(), "DbProject::export".to_string())
            })?;

            let bytes = storage_client
                .object()
                .download("ems-candidate-profile", &candidate.image_file)
                .await
                .map_err(|err| {
                    Error::InternalServerError(
                        err.to_string(),
                        format!("DbProject::export: {}", candidate.image_file),
                    )
                })?;

            zip.write_all(&bytes).map_err(|e| {
                Error::InternalServerError(e.to_string(), "DbProject::export".to_string())
            })?;
        }

        zip.finish().map_err(|e| {
            Error::InternalServerError(e.to_string(), "DbProject::export".to_string())
        })?;

        Ok(buf)
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

#[async_trait]
impl Authorize for DbProject {
    async fn authorize(
        &self,
        user_id: Uuid,
        pool: &sqlx::PgPool,
        action: ActionType,
    ) -> Result<()> {
        // If user is owner of project or is member of project
        let is_owner = sqlx::query!(
            r#"
                SELECT COUNT(*)
                FROM projects
                WHERE id = $1 AND owner_id = $2
                "#,
            self.id,
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
            self.id,
            user_id
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0)
            > 0;

        match action {
            ActionType::Create | ActionType::Update => {
                if is_owner || is_member {
                    Ok(())
                } else {
                    Err(Error::InvalidPermission(
                        "Project Authorizer".to_string(),
                        "User is not authorized to create/update project".to_string(),
                    ))
                }
            }
            ActionType::Delete => {
                if is_owner {
                    Ok(())
                } else {
                    Err(Error::InvalidPermission(
                        "Project Authorizer".to_string(),
                        "User is not authorized to delete project".to_string(),
                    ))
                }
            }

            _ => Ok(()),
        }
    }
}
