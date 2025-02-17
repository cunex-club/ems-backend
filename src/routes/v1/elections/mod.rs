use actix_web::web::ServiceConfig;

pub mod get_election_details;
pub mod query_election;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_election_details::get_election_details);
}
