mod auth;
mod password_reset;
mod verify;


use actix_identity::Identity;

use actix_web::{
    get,
    web::{self, Data}, HttpResponse, Responder,
};



use tracing::{instrument, warn};

use crate::{
    models::{
        user::{self},
        MongoDatabase,
    },
};
use auth::*;
use password_reset::*;
use verify::*;

pub fn factory(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(login)
            .service(signup)
            .service(get_user)
            .service(check_username)
            .service(password_reset)
            .service(change_password)
            .service(validate_password_reset)
            .service(logout)
            .service(user_verfication),
    );
}

#[get("me")]
#[instrument(skip( db,id),fields(user_id = %id.id().unwrap()))]
pub async fn get_user(id: Identity, db: Data<MongoDatabase>) -> impl Responder {
    match user::logic::get_user(&db, id.id().unwrap()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            warn!("Failed to get user: {}", err);
            HttpResponse::InternalServerError().body("Failed to get user")
        }
    }
}
