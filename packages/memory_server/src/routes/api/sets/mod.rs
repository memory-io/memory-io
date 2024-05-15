mod modify;
use std::str::FromStr;

use actix_identity::Identity;

use actix_web::web::Query;
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};

use mongodb::{bson::oid::ObjectId};

use serde::Deserialize;
use tracing::{debug, error, instrument};


use crate::models::set;
use crate::models::{
    set::model::{SetVisibility},
    MongoDatabase,
};
use modify::*;

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

#[get("/{id}")]
#[instrument(skip(db,id), fields(user_id = %id.as_ref().map(|a| a.id().unwrap().to_string()).unwrap_or("None".to_string()), set_id = %set_id))]
pub async fn get_set(
    id: Option<Identity>,
    set_id: web::Path<String>,
    db: Data<MongoDatabase>,
    options: Query<GetSetsOptions>,
) -> impl Responder {
    debug!("Getting set options: {:?}", options);

    let Ok(set_id) = ObjectId::from_str(&set_id) else {
        return HttpResponse::BadRequest().body("Invalid Set ID");
    };
    match set::get_set(&db, &set_id, options.include_users, options.include_cards).await {
        Err(err) => {
            error!("Failed to get set: {}", err);
            HttpResponse::InternalServerError().body("Failed to get set")
        }
        Ok(Some(a)) => {
            if let SetVisibility::Private = a.visibility {
                if let Some(id) = id {
                    if let Ok(user_id) = id.id() {
                        if a.user_id.to_hex() != user_id {
                            debug!("Not Authorized to view this set");
                            return HttpResponse::Unauthorized()
                                .body("Not Authorized to view this set");
                        }
                    }
                } else {
                    debug!("Not Authorized to view this set");
                    return HttpResponse::Unauthorized().body("Not Authorized to view this set");
                }
            }
            debug!("Found set");
            HttpResponse::Ok().json(a)
        }
        Ok(None) => {
            debug!("Set not found");
            return HttpResponse::NotFound().body("Set not found");
        }
    }
}

#[get("")]
#[instrument(skip(db, id), fields(user_id = %id.id().unwrap()))]
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

#[get("/recents")]
#[instrument(skip(db))]
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
