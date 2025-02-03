use crate::models::question::{
    db::DbQuestion,
    fetch_levels::{default::DefaultQuestion, id_only::IdOnlyQuestion},
};

use mysk_lib::models::top_level_variant::TopLevelVariant;

pub(crate) mod db;
pub(crate) mod fetch_levels;

pub type Question =
    TopLevelVariant<DbQuestion, IdOnlyQuestion, IdOnlyQuestion, DefaultQuestion, DefaultQuestion>;
