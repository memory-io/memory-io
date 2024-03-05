use actix_web::web;

mod set;

pub fn factory(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sets")
            .service(set::create_set)
            .service(set::get_set)
            .service(set::get_sets)
            .service(set::delete_set)
            .service(set::patch_set),
            
    );
}
