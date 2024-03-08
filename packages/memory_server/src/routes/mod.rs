pub mod api;
pub mod user;

use actix_web::web;


pub fn factory(cfg: &mut web::ServiceConfig) {

    cfg.service(web::scope("/api").configure(api::factory).service(
        web::scope("/users")
            .service(user::login)
            .service(user::signup)
            .service(user::get_user)
            .service(user::check_username),

    ));
}
