use std::{process::id, str::FromStr};

use actix_identity::Identity;

use actix_session::Session;
use actix_web::{
    delete, get, patch, post,
    web::{self, Data, Json},
    Handler, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use futures_util::TryFutureExt;
use mongodb::{bson::oid::ObjectId, error::WriteFailure};

use serde::Deserialize;
use tracing::{debug, error, info, warn};

use crate::{
    models::{
        card::{self, Card}, set::{self, CreateSet, PatchSet, Set, SetVisibility}, user::{self, UserSignup}, MongoDatabase
    },
    startup::ServerConfig,
};

#[derive(Deserialize)]
pub struct CreateSetRequest {
    pub title: String,
    pub visibility: SetVisibility,
}


#[delete("/{id}")]
pub async fn delete_set(
    id: Identity,
    set_id: web::Path<String>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    let id = ObjectId::from_str(&id.id().unwrap()).unwrap();
    return match set::delete_set(&db, &ObjectId::from_str(&set_id).unwrap(), &id).await {
        Err(err) => {
            error!("Failed to get set: {}", err);
            HttpResponse::InternalServerError().body("Failed to get set")
        }
        Ok(_) => HttpResponse::Ok().await.unwrap(),
    };
}

#[patch("/{id}")]
pub async fn patch_set(
    db: Data<MongoDatabase>,
    id: Identity,
    set_id: web::Path<String>,
    action: web::Json<PatchSet>,
) -> impl Responder {
    let user_id = ObjectId::from_str(&id.id().unwrap()).unwrap();
    let set_id = ObjectId::from_str(&set_id).unwrap();
    match &*action{
        PatchSet::AddCard{front, back} => {
            return match card::add_card_to_set(&db, &set_id, &user_id,Card{id: bson::Uuid::new(),front:front.to_string(),back:back.to_string()}).await {
                Ok(_) => HttpResponse::Ok().await.unwrap(),
                Err(err) => {
                    error!("Failed to add card: {}", err);
                    HttpResponse::InternalServerError().body("Failed to add card")
                }
            };
        }
        PatchSet::RemoveCard{id} => {
            return match card::remove_card_from_set(&db, &set_id, &user_id, &id).await {
                Ok(_) => HttpResponse::Ok().await.unwrap(),
                Err(err) => {
                    error!("Failed to remove card: {}", err);
                    HttpResponse::InternalServerError().body("Failed to remove card")
                }
            };
        }
        PatchSet::UpdateCard(card) => {
            return match card::update_card_in_set(&db, &set_id, &user_id, card).await {
                Ok(true) => HttpResponse::Ok().await.unwrap(),
                Ok(false) => HttpResponse::NotFound().body("Card not found"),
                Err(err) => {
                    error!("Failed to update card: {}", err);
                    HttpResponse::InternalServerError().body("Failed to update card")
                }
            };
        }
    
    }
}

#[get("")]
pub async fn get_sets(id: Identity, db: Data<MongoDatabase>) -> impl Responder {
    let user_id = ObjectId::from_str(&id.id().unwrap()).unwrap();
    return match set::get_sets_from_user(&db, &user_id, 10).await {
        Ok(sets) => HttpResponse::Ok().json(sets),
        Err(err) => {
            error!("Error getting sets {err:?}");
            HttpResponse::InternalServerError().await.unwrap()
        }
    };
}

#[post("/create")]
pub async fn create_set(
    id: Identity,
    set: Json<CreateSetRequest>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    let user_id = ObjectId::from_str(id.id().unwrap().as_str()).unwrap();

    return match set::create_set(
        &db,
        CreateSet {
            user_id,
            title: set.0.title,
            visibility: set.0.visibility,
            cards: Vec::new(),
        },
    )
    .await
    {
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
            error!("Failed to create user: {}", err);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
        Ok(a) => {
            return match set::get_set_with_cards(&db, &a.inserted_id.as_object_id().unwrap()).await {
                Ok(Some(set)) => HttpResponse::Ok().json(set),
                Ok(None) => {
                    error!("Created Set not found");
                    HttpResponse::InternalServerError().body("Failed to create set")
                }
                Err(err) => {
                    error!("Failed to create set: {}", err);
                    HttpResponse::InternalServerError().body("Failed to create set")
                }
            };
        }
    };
}

#[get("/{id}")]
pub async fn get_set(
    id: Option<Identity>,
    set_id: web::Path<String>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    return match set::get_set_with_cards(&db, &ObjectId::from_str(&set_id).unwrap()).await {
        Err(err) => {
            error!("Failed to get set: {}", err);
            HttpResponse::InternalServerError().body("Failed to get set")
        }
        Ok(Some(a)) => {
            debug!("Set: {:?}", a);
            if let SetVisibility::Private = a.visibility {
                if let Some(id) = id {
                    if let Ok(user_id) = id.id() {
                        if a.user_id.to_hex() != user_id {
                            return HttpResponse::Unauthorized()
                                .body("Not Authorized to view this set");
                        }
                    }
                } else {
                    return HttpResponse::Unauthorized().body("Not Authorized to view this set");
                }
            }
            HttpResponse::Ok().json(a)
        }
        Ok(None) => HttpResponse::NotFound().body("Set not found"),
    };
}



#[get("/recents")]
pub async fn get_recent_sets(
    db: Data<MongoDatabase>,
) -> impl Responder {
    return match set::get_most_recent_public_sets(&db, 20).await {
        Err(err) => {
            error!("Failed to get set: {}", err);
            HttpResponse::InternalServerError().body("Failed to get set")
        }
        Ok(sets) => {
            debug!("Set: {:?}", sets);
            
            HttpResponse::Ok().json(sets)
        }
    };
}
