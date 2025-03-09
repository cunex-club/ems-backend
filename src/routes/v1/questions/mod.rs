use actix_web::web::ServiceConfig;

pub mod create_question;
pub mod get_question_details;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_question_details::get_question_details);
    cfg.service(create_question::create_question);
}
