use actix_web::web::ServiceConfig;

pub mod create_candidate;
pub mod delete_candidate;
pub mod get_candidate_details;
pub mod modify_candidate;
pub mod query_candidates;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_candidate::create_candidate);
    cfg.service(delete_candidate::delete_candidate);
    cfg.service(get_candidate_details::get_candidate_details);
    cfg.service(query_candidates::query_candidates);
    cfg.service(modify_candidate::modify_candidate);
}
