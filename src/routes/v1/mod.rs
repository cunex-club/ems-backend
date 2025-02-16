use actix_web::web::{scope, ServiceConfig};

pub mod elections;
pub mod projects;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/projects").configure(projects::config));
    cfg.service(scope("/elections").configure(elections::config));
}
