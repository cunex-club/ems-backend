use actix_web::web::ServiceConfig;

pub mod create_candidate;
pub mod delete_candidate;
pub mod get_candidate_details;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(create_candidate::create_candidate);
    // cfg.service(delete_candidate::delete_candidate);
    cfg.service(get_candidate_details::get_candidate_details);
}
