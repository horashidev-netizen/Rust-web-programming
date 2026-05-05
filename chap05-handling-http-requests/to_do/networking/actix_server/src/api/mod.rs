use actix_web::web::ServiceConfig;
pub mod basic_actions;

pub fn view_factory(app: &mut ServiceConfig) {
    basic_actions::basic_action_factory(app)
}