use crate::models::question::{
    db::DbQuestion,
    fetch_levels::{default::DefaultQuestion, id_only::IdOnlyQuestion},
};

use mysk_lib::models::{top_level_variant::TopLevelVariant, traits::TopLevelQuery};
use requests::{queryable::QueryableQuestion, sortable::SortableQuestion};

pub(crate) mod db;
pub(crate) mod fetch_levels;
pub(crate) mod requests;

pub type Question =
    TopLevelVariant<DbQuestion, IdOnlyQuestion, IdOnlyQuestion, DefaultQuestion, DefaultQuestion>;

impl TopLevelQuery<DbQuestion, QueryableQuestion, SortableQuestion> for Question {}
