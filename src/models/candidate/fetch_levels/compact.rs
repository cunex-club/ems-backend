use crate::models::candidate::db::DbCandidate;
use mysk_lib::common::string::MultiLangString;
use mysk_lib_macros::impl_fetch_level_variant_from;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactCandidate {
    pub id: Uuid,
    pub choice_label: MultiLangString,
    pub title: String,
}

impl From<DbCandidate> for CompactCandidate {
    fn from(candidate: DbCandidate) -> Self {
        Self {
            id: candidate.id,
            choice_label: MultiLangString {
                th: candidate.choice_label_th,
                en: Some(candidate.choice_label_en),
            },
            title: candidate.title,
        }
    }
}

impl_fetch_level_variant_from!(candidate, Compact, CompactCandidate, DbCandidate);
