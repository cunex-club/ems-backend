use actix_web::web::ServiceConfig;

pub mod create_project;
pub mod delete_project;
pub mod get_projects_details;
pub mod modify_project;
pub mod query_projects;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_projects_details::get_projects_details);
    cfg.service(query_projects::query_projects);
    cfg.service(create_project::create_project);
    cfg.service(modify_project::modify_project);
    cfg.service(delete_project::delete_project);
}
