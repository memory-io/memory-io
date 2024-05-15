

use actix_identity::Identity;

use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};


use crate::{
    models::{
        user::{self, model::UserSignup},
        MongoDatabase,
    },
};
use serde::Deserialize;

use tracing::{debug, info, instrument, trace, warn};
use user::logic::auth;

#[get("check_username/{username}")]
#[instrument(skip(db))]
pub async fn check_username(db: Data<MongoDatabase>, username: Path<String>) -> impl Responder {
    match auth::check_username(&db, &username).await {
        Ok(_) => HttpResponse::Ok().await.unwrap(),
        Err(a) => HttpResponse::Conflict().json(a.to_string()),
    }
}

#[derive(Deserialize, Debug)]
struct UserLogin {
    email: String,
    password: String,
}

#[post("/login")]
#[instrument(skip(user, request, db), fields(user.email = %user.email))]
pub async fn login(
    request: HttpRequest,
    user: Json<UserLogin>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    debug!("User {user:?}");
    return match auth::authenticate_user(&db, &user.email, &user.password).await {
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
#[post("/logout")]
#[instrument(skip(id),fields(user_id = id.id().unwrap()))]
pub async fn logout(id: Identity) -> impl Responder {
    id.logout();

    HttpResponse::Ok()
}

#[instrument(skip(db,user,request),fields(user.email = %user.email,user.username = %user.username))]
#[post("/signup")]
pub async fn signup(
    request: HttpRequest,
    user: Json<UserSignup>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    info!("User {user:?}");
    return match auth::create_user(&db, user.0).await {
        Err(err) => {
            warn!("Failed to create user: {}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
        Ok(a) => {
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
