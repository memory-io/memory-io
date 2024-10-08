use std::mem;

use actix_web::web;
pub mod folders;
pub mod memorize;
mod sets;
pub mod users;
pub fn factory(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(sets::factory)
            .configure(users::factory)
            .configure(folders::factory)
            .configure(memorize::factory),
    );
}
