use chrono::{DateTime, Utc};
use mysk_lib::prelude::*;
use mysk_lib_derives::{BaseQuery, GetById};
use mysk_lib_macros::traits::db::{BaseQuery, GetById};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool};
use uuid::Uuid;

use crate::models::auth::oauth::GoogleUserResult;

#[derive(BaseQuery, Clone, Debug, Deserialize, FromRow, GetById, Serialize)]
#[base_query(query = "SELECT * FROM users")]
pub struct DbUser {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub email: String,
    pub username: String,
    pub profile: String,
    pub first_name: String,
    pub last_name: String,
    pub is_admin: bool,
}

impl DbUser {
    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<Uuid>> {
        let user_id = query!("SELECT id FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await?;

        Ok(user_id.map(|id| id.id))
    }

    pub async fn create_user_from_google(
        pool: &PgPool,
        google_user: GoogleUserResult,
    ) -> Result<Self> {
        query_as!(DbUser, "INSERT INTO users (username, email, profile, first_name, last_name) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            google_user.name,
            google_user.email,
            google_user.picture,
            google_user.given_name,
            google_user.family_name
    )
            .fetch_one(pool)
            .await
            .map_err(|err| {
                Error::InternalSeverError(err.to_string(), "/auth/oauth/gsi".to_string())
            })
    }
}
