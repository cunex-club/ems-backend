use actix_web::web::ServiceConfig;

pub mod add_member;
pub mod create_project;
pub mod delete_member;
pub mod delete_project;
pub mod export_project;
pub mod get_projects_details;
pub mod modify_project;
pub mod query_projects;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_projects_details::get_projects_details);
    cfg.service(query_projects::query_projects);
    cfg.service(create_project::create_project);
    cfg.service(modify_project::modify_project);
    cfg.service(delete_project::delete_project);
    cfg.service(add_member::add_member);
    cfg.service(delete_member::delete_member);
    cfg.service(export_project::export_project);
}
