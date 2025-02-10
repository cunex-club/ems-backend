use crate::models::election::db::DbElection;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyElection {
    pub id: Uuid,
}

impl_id_only_variant_from!(election, IdOnlyElection, DbElection);
