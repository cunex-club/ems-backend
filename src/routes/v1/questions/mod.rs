use actix_web::web::ServiceConfig;

pub mod get_question_details;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_question_details::get_question_details);
}
