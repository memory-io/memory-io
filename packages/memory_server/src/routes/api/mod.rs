use actix_web::web;
mod sets;
pub mod users;

pub fn factory(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(sets::factory)
            .configure(users::factory),
    );
}
