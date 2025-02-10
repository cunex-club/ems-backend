use crate::models::candidate::db::DbCandidate;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyCandidate {
    pub id: Uuid,
}

impl_id_only_variant_from!(candidates, IdOnlyCandidate, DbCandidate);
