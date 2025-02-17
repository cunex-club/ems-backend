use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableElection {
    Id,
    CreatedAt,
    NameTh,
    NameEn,
    Label,
    HeaderTh,
    HeaderEn,
    DetailTh,
    DetailEn,
}

impl Default for SortableElection {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableElection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::NameTh => write!(f, "name_th"),
            Self::NameEn => write!(f, "name_en"),
            Self::Label => write!(f, "label"),
            Self::HeaderTh => write!(f, "header_th"),
            Self::HeaderEn => write!(f, "header_en"),
            Self::DetailTh => write!(f, "detail_th"),
            Self::DetailEn => write!(f, "detail_en"),
        }
    }
}
