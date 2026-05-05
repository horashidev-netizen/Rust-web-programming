use actix_web::web::{get, scope, ServiceConfig};


mod create;
mod get;
mod delete;
mod update;

pub fn basic_action_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .route("get/all", get().to(get::get_all))
    );
}