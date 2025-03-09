use actix_web::web::ServiceConfig;

pub mod create_question;
pub mod delete_question;
pub mod get_question_details;
pub mod modify_question;
pub mod query_question;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_question_details::get_question_details);
    cfg.service(create_question::create_question);
    cfg.service(query_question::query_question);
    cfg.service(modify_question::modify_question);
    cfg.service(delete_question::delete_question);
}
