use std::str::FromStr;

use actix_identity::Identity;

use actix_web::{
    get, post,
    web::{self, Data, Form, Json, Path},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use mongodb::error::{Error, WriteFailure};

use serde::Deserialize;
use tracing::{debug, info, trace, warn};

use crate::models::{
    user::{self, UserSignup},
    MongoDatabase,
};

pub fn factory(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(login)
            .service(signup)
            .service(get_user)
            .service(check_username),
    );
}

#[post("/signup")]
pub async fn signup(
    request: HttpRequest,
    user: Json<UserSignup>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    info!("User {user:?}");
    return match user::create_user(&db, user.0).await {
        Err(err) => {
            match &*err.kind {
                mongodb::error::ErrorKind::Write(err) => {
                    if let WriteFailure::WriteError(write_err) = err {
                        if write_err.code == 11000 {
                            return HttpResponse::Conflict().body("User already exists");
                        }
                    }
                }
                _ => {}
            }
            warn!("Failed to create user: {}", err);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
        Ok(a) => {
            //session.insert("user_id", a.inserted_id.as_object_id().unwrap().to_hex()).unwrap();
            trace!("User Signed Up");
            Identity::login(
                &request.extensions(),
                a.inserted_id.as_object_id().unwrap().to_hex(),
            )
            .unwrap();
            HttpResponse::Ok().into()
        }
    };
}

#[get("me")]
pub async fn get_user(id: Identity, db: Data<MongoDatabase>) -> impl Responder {
    return match user::get_user(&db, id.id().unwrap()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            warn!("Failed to get user: {}", err);
            HttpResponse::InternalServerError().body("Failed to get user")
        }
    };
}

#[get("check_username/{username}")]
pub async fn check_username(db: Data<MongoDatabase>, username: Path<String>) -> impl Responder {
    return match user::check_username(&db, &username).await {
        Ok(_) => HttpResponse::Ok().await.unwrap(),
        Err(a) => match a.get_custom::<String>() {
            Some(err) => HttpResponse::Conflict().json(err),
            None => HttpResponse::InternalServerError().await.unwrap(),
        },
    };
}

#[derive(Deserialize, Debug)]
struct UserLogin {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    request: HttpRequest,
    user: Json<UserLogin>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    debug!("User {user:?}");
    return match user::authenticate_user(&db, &user.email, &user.password).await {
        Err(err) => {
            warn!("Failed to authenticate user: {}", err);
            HttpResponse::InternalServerError().body("Failed to authenticate user")
        }
        Ok(Some(a)) => {
            Identity::login(&request.extensions(), a.to_hex()).unwrap();
            HttpResponse::Ok().into()
        }
        Ok(None) => HttpResponse::Unauthorized().body("Failed to authenticate user"),
    };
}
