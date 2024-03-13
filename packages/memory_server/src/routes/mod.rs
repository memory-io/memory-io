pub mod api;

use actix_web::web;

pub fn factory(cfg: &mut web::ServiceConfig) {
    cfg.configure(api::factory);
}
