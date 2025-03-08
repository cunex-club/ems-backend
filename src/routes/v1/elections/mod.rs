use actix_web::web::ServiceConfig;

pub mod create_election;
pub mod get_election_details;
pub mod modify_election;
pub mod query_election;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_election_details::get_election_details);
    cfg.service(query_election::query_election);
    cfg.service(create_election::create_election);
    cfg.service(modify_election::modify_election);
}
