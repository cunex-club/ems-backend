use crate::models::project::db::DbProject;
use async_trait::async_trait;
use mysk_lib::permissions::Authorizer;
use mysk_lib::prelude::*;
use mysk_lib::{common::requests::FetchLevel, models::traits::FetchLevelVariant};
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyProject {
    pub id: Uuid,
}

impl_id_only_variant_from!(project, IdOnlyProject, DbProject);
