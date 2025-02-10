use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableProject {
    Id,
    CreatedAt,
    Name,
}

impl Default for SortableProject {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableProject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::Name => write!(f, "name"),
        }
    }
}
