use crate::models::project::db::DbProject;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyProject {
    pub id: Uuid,
}

impl_id_only_variant_from!(project, IdOnlyProject, DbProject);
