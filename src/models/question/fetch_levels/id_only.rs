use crate::models::question::db::DbQuestion;
use mysk_lib_macros::impl_id_only_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdOnlyQuestion {
    pub id: Uuid,
}

impl_id_only_variant_from!(question, IdOnlyQuestion, DbQuestion);
