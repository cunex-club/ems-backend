use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortableQuestion {
    Id,
    CreatedAt,
    Label,
    FacultyCode,
    StudentProgram,
    ElectionId,
}

impl Default for SortableQuestion {
    fn default() -> Self {
        Self::Id
    }
}

impl Display for SortableQuestion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::Label => write!(f, "label"),
            Self::FacultyCode => write!(f, "faculty_code"),
            Self::StudentProgram => write!(f, "student_program"),
            Self::ElectionId => write!(f, "election_id"),
        }
    }
}
