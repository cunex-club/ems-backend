use crate::prelude::*;
use async_trait::async_trait;
use sqlx::{query, PgPool};
use std::sync::Arc;

pub enum ActionType {
    Create,
    ReadIdOnly,
    ReadCompact,
    ReadDefault,
    ReadDetailed,
    Update,
    Delete,
}

#[allow(unused_variables)]
#[async_trait]
pub trait Authorizer: Send + Sync {
    // fn authorize_candidates(
    //     &self,
    //     candidate: &DbCandidate,
    //     pool: &PgPool,
    //     action: ActionType,
    // ) -> Result<()>;

    fn clone_to_arc(&self) -> Arc<dyn Authorizer>;
}

pub struct DefaultAuthorizer;

impl Authorizer for DefaultAuthorizer {
    fn clone_to_arc(&self) -> Arc<dyn Authorizer> {
        Arc::new(DefaultAuthorizer)
    }
}
