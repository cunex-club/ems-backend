use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableCandidate {
    Id,
    CreatedAt,
    Title,
    ChoiceLabelTh,
    ChoiceLabelEn,
}

impl Default for SortableCandidate {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableCandidate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::Title => write!(f, "title"),
            Self::ChoiceLabelTh => write!(f, "choice_label_th"),
            Self::ChoiceLabelEn => write!(f, "choice_label_en"),
        }
    }
}
