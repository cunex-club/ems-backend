use crate::models::election::{
    db::DbElection,
    fetch_levels::{compact::CompactElection, default::DefaultElection, id_only::IdOnlyElection},
};

use mysk_lib::models::top_level_variant::TopLevelVariant;

use super::Authorize;

pub(crate) mod db;
pub(crate) mod fetch_levels;

pub type Election =
    TopLevelVariant<DbElection, IdOnlyElection, CompactElection, DefaultElection, DefaultElection>;
