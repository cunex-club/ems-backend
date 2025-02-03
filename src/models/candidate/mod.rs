use crate::models::candidate::{
    db::DbCandidate,
    fetch_levels::{
        compact::CompactCandidate, default::DefaultCandidate, id_only::IdOnlyCandidate,
    },
};

use mysk_lib::models::{top_level_variant::TopLevelVariant, traits::TopLevelQuery};

pub(crate) mod db;
pub(crate) mod fetch_levels;

pub type Candidate = TopLevelVariant<
    DbCandidate,
    IdOnlyCandidate,
    CompactCandidate,
    DefaultCandidate,
    DefaultCandidate,
>;
