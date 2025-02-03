pub(crate) mod db;

use crate::models::user::db::DbUser;
use chrono::{DateTime, Utc};
use mysk_lib::prelude::*;
use mysk_lib_macros::traits::db::GetById;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::auth::oauth::GoogleUserResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub email: String,
    pub username: String,
    pub profile: String,
    pub first_name: String,
    pub last_name: String,
    pub is_admin: bool,
}

impl User {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Self> {
        let user = DbUser::get_by_id(pool, id).await?;

        Ok(Self {
            id: user.id,
            created_at: user.created_at,
            email: user.email,
            username: user.username,
            profile: user.profile,
            first_name: user.first_name,
            last_name: user.last_name,
            is_admin: user.is_admin,
        })
    }

    pub async fn get_by_ids(pool: &PgPool, ids: Vec<Uuid>) -> Result<Vec<Self>> {
        let users = DbUser::get_by_ids(pool, ids).await?;

        Ok(users
            .into_iter()
            .map(|user| Self {
                id: user.id,
                created_at: user.created_at,
                email: user.email,
                username: user.username,
                profile: user.profile,
                first_name: user.first_name,
                last_name: user.last_name,
                is_admin: user.is_admin,
            })
            .collect())
    }

    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>> {
        let user_id = DbUser::get_by_email(pool, email).await?;

        match user_id {
            Some(user_id) => {
                let user = Self::get_by_id(pool, user_id).await?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    pub async fn create_user_from_google(
        pool: &PgPool,
        google_user: GoogleUserResult,
    ) -> Result<Self> {
        let user = DbUser::create_user_from_google(pool, google_user).await?;

        Ok(Self {
            id: user.id,
            created_at: user.created_at,
            email: user.email,
            username: user.username,
            profile: user.profile,
            first_name: user.first_name,
            last_name: user.last_name,
            is_admin: user.is_admin,
        })
    }
}
