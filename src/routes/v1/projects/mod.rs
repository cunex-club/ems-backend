use actix_web::web::ServiceConfig;

pub mod get_projects_details;
pub mod query_projects;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_projects_details::get_projects_details);
}
