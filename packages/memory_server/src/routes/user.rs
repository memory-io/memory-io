use actix_identity::Identity;

use actix_session::Session;
use actix_web::{
    post,
    web::{Data, Form, Json},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use mongodb::error::WriteFailure;

use serde::Deserialize;
use tracing::{debug, info, trace, warn};

use crate::models::{
    user::{self, UserSignup},
    MongoDatabase,
};

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
