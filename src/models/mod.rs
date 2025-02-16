use async_trait::async_trait;
use mysk_lib::{permissions::ActionType, prelude::*};
use sqlx::PgPool;
use uuid::Uuid;

pub(crate) mod auth;
pub(crate) mod candidate;
pub(crate) mod election;
pub(crate) mod project;
pub(crate) mod question;
pub(crate) mod user;

#[async_trait]
pub trait Authorize {
    async fn authorize(&self, user_id: Uuid, pool: &PgPool, action: ActionType) -> Result<()>;
}
