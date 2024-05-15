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
use tracing::{debug, error, instrument};

use crate::models::card::CardNoID;
use crate::models::set;
use crate::models::{
    card::{self, Card},
    set::model::{CreateSet, PatchSet, SetVisibility},
    MongoDatabase,
};

#[derive(Deserialize, Debug)]
pub struct CreateSetRequest {
    pub title: String,
    pub description: Option<String>,
    pub visibility: SetVisibility,
    #[serde(default)]
    pub cards: Vec<CardNoID>,
}

#[delete("/{id}")]
#[instrument(skip(db,id,set_id), fields(set_id = %set_id, user_id = %id.id().unwrap()))]
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
#[instrument(skip(db,id,set_id), fields(set_id = %set_id, user_id = %id.id().unwrap()))]
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
        PatchSet::UpdateSet(set) => match set::update_set(&db, &set_id, &user_id, set).await {
            Ok(true) => HttpResponse::Ok().await.unwrap(),
            Ok(false) => HttpResponse::NotFound().body("Set not found"),
            Err(err) => {
                error!("Failed to update set: {}", err);
                HttpResponse::InternalServerError().body("Failed to update set")
            }
        },
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


#[post("/create")]
#[instrument(skip(db, id),fields(user_id = %id.id().unwrap()))]
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
            description: set.0.description,
            visibility: set.0.visibility,
            cards: set
                .0
                .cards
                .into_iter()
                .map(|a| Card {
                    id: bson::Uuid::new(),
                    front: a.front,
                    back: a.back,
                })
                .collect(),
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
            match set::get_set(&db, &a.inserted_id.as_object_id().unwrap(), false, false).await {
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
