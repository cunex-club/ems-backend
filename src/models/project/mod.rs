use crate::models::project::{
    db::DbProject,
    fetch_levels::{compact::CompactProject, default::DefaultProject, id_only::IdOnlyProject},
};

use mysk_lib::models::top_level_variant::TopLevelVariant;

pub(crate) mod db;
pub(crate) mod fetch_levels;

pub type Project =
    TopLevelVariant<DbProject, IdOnlyProject, CompactProject, DefaultProject, DefaultProject>;
