use std::time::Duration;

use actix_identity::Identity;

use actix_web::{
    get, post,
    web::{self, Data, Json, Path},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};

use mongodb::error::WriteFailure;
use serde::Deserialize;
use tokio::time::sleep;
use tracing::{debug, info, trace, warn};

use crate::{
    models::{
        user::{self, model::UserSignup},
        MongoDatabase,
    },
    startup::EmailClient,
};

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
            .service(logout),
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

#[get("me")]
pub async fn get_user(id: Identity, db: Data<MongoDatabase>) -> impl Responder {
    match user::get_user(&db, id.id().unwrap()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            warn!("Failed to get user: {}", err);
            HttpResponse::InternalServerError().body("Failed to get user")
        }
    }
}

#[derive(Deserialize, Debug)]
struct ResetRequest {
    email: String,
}
#[post("password_reset")]
pub async fn password_reset(
    email: Json<ResetRequest>,
    db: Data<MongoDatabase>,
    client: Data<EmailClient>,
) -> impl Responder {
    match user::password_reset(&db, &client, &email.email).await {
        Ok(_) => HttpResponse::Ok().await.unwrap(),
        Err(err) => {
            warn!("Failed to send reset {err}");
            sleep(Duration::from_millis(500)).await;
            HttpResponse::Ok().await.unwrap()
        }
    }
}

#[get("password_reset/{token}")]
pub async fn validate_password_reset(
    token: Path<String>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    match user::validate_reset(&db, &token).await {
        Ok(_) => HttpResponse::Ok().await.unwrap(),
        Err(err) => {
            warn!("Failed to validate reset {err}");
            HttpResponse::InternalServerError().await.unwrap()
        }
    }
}
#[derive(Deserialize, Debug)]
struct PasswordReset {
    token: String,
    password: String,
}

#[post("change_password")]
pub async fn change_password(
    reset: Json<PasswordReset>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    match user::change_password(&db, &reset.token, &reset.password).await {
        Ok(()) => HttpResponse::Ok().await.unwrap(),
        Err(err) => {
            warn!("Failed to change password {err}");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[get("check_username/{username}")]
pub async fn check_username(db: Data<MongoDatabase>, username: Path<String>) -> impl Responder {
    match user::check_username(&db, &username).await {
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
#[post("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.logout();

    return HttpResponse::Ok();
}
