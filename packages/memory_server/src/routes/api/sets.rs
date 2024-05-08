use std::str::FromStr;

use actix_identity::Identity;

use actix_web::web::Query;
use actix_web::{
    delete, get, patch, post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};

use mongodb::{bson::oid::ObjectId, error::WriteFailure};

use serde::Deserialize;
use tracing::{debug, error};

use crate::models::set;
use crate::models::{
    card::{self, Card},
    set::model::{CreateSet, PatchSet, SetVisibility},
    MongoDatabase,
};

pub fn factory(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sets")
            .service(create_set)
            .service(get_recent_sets)
            .service(get_set)
            .service(get_sets)
            .service(delete_set)
            .service(patch_set),
    );
}

#[derive(Deserialize, Debug, Default)]
struct GetSetsOptions {
    #[serde(rename = "includeUser")]
    #[serde(default)]
    include_users: bool,
    #[serde(default)]
    #[serde(rename = "includeCards")]
    include_cards: bool,

    limit: Option<usize>,
}

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
    match set::delete_set(&db, &ObjectId::from_str(&set_id).unwrap(), &id).await {
        Err(err) => {
            error!("Failed to get set: {}", err);
            HttpResponse::InternalServerError().body("Failed to get set")
        }
        Ok(_) => HttpResponse::Ok().await.unwrap(),
    }
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
    match &*action {
        PatchSet::AddCard { front, back } => {
            match card::add_card_to_set(
                &db,
                &set_id,
                &user_id,
                Card {
                    id: bson::Uuid::new(),
                    front: front.to_string(),
                    back: back.to_string(),
                },
            )
            .await
            {
                Ok(_) => HttpResponse::Ok().await.unwrap(),
                Err(err) => {
                    error!("Failed to add card: {}", err);
                    HttpResponse::InternalServerError().body("Failed to add card")
                }
            }
        }
        PatchSet::RemoveCard { id } => {
            match card::remove_card_from_set(&db, &set_id, &user_id, id).await {
                Ok(_) => HttpResponse::Ok().await.unwrap(),
                Err(err) => {
                    error!("Failed to remove card: {}", err);
                    HttpResponse::InternalServerError().body("Failed to remove card")
                }
            }
        }
        PatchSet::UpdateCard(card) => {
            match card::update_card_in_set(&db, &set_id, &user_id, card).await {
                Ok(true) => HttpResponse::Ok().await.unwrap(),
                Ok(false) => HttpResponse::NotFound().body("Card not found"),
                Err(err) => {
                    error!("Failed to update card: {}", err);
                    HttpResponse::InternalServerError().body("Failed to update card")
                }
            }
        }
    }
}

#[get("")]
pub async fn get_sets(
    id: Identity,
    db: Data<MongoDatabase>,
    options: Query<GetSetsOptions>,
) -> impl Responder {
    let user_id = ObjectId::from_str(&id.id().unwrap()).unwrap();

    match set::get_sets_from_user(
        &db,
        &user_id,
        options.limit.unwrap_or(10) as i64,
        options.include_users,
        options.include_cards,
    )
    .await
    {
        Ok(sets) => HttpResponse::Ok().json(sets),
        Err(err) => {
            error!("Error getting sets {err:?}");
            HttpResponse::InternalServerError().await.unwrap()
        }
    }
}

#[post("/create")]
pub async fn create_set(
    id: Identity,
    set: Json<CreateSetRequest>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    let user_id = ObjectId::from_str(id.id().unwrap().as_str()).unwrap();

    match set::create_set(
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
            match set::get_set(&db, &a.inserted_id.as_object_id().unwrap(), false, false)
                .await
            {
                Ok(Some(set)) => HttpResponse::Ok().json(set),
                Ok(None) => {
                    error!("Created Set not found");
                    HttpResponse::InternalServerError().body("Failed to create set")
                }
                Err(err) => {
                    error!("Failed to create set: {}", err);
                    HttpResponse::InternalServerError().body("Failed to create set")
                }
            }
        }
    }
}

#[get("/{id}")]
pub async fn get_set(
    id: Option<Identity>,
    set_id: web::Path<String>,
    db: Data<MongoDatabase>,
    options: Query<GetSetsOptions>,
) -> impl Responder {
    debug!("Getting set options: {:?}", options);
    match set::get_set(
        &db,
        &ObjectId::from_str(&set_id).unwrap(),
        options.include_users,
        options.include_cards,
    )
    .await
    {
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
    }
}

#[get("/recents")]
pub async fn get_recent_sets(db: Data<MongoDatabase>) -> impl Responder {
    match set::get_most_recent_public_sets(&db, 20).await {
        Err(err) => {
            error!("Failed to get set: {}", err);
            HttpResponse::InternalServerError().body("Failed to get set")
        }
        Ok(sets) => {
            debug!("Set: {:?}", sets);

            HttpResponse::Ok().json(sets)
        }
    }
}


