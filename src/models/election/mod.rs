use crate::models::election::{
    db::DbElection,
    fetch_levels::{compact::CompactElection, default::DefaultElection, id_only::IdOnlyElection},
};

use mysk_lib::models::{top_level_variant::TopLevelVariant, traits::TopLevelQuery};
use requests::{queryable::QueryableElection, sortable::SortableElection};

pub(crate) mod db;
pub(crate) mod fetch_levels;
pub(crate) mod requests;
pub type Election =
    TopLevelVariant<DbElection, IdOnlyElection, CompactElection, DefaultElection, DefaultElection>;

impl TopLevelQuery<DbElection, QueryableElection, SortableElection> for Election {}
