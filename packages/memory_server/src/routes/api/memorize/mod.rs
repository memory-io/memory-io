use std::str::FromStr;

use actix_identity::Identity;

use actix_web::{
    get, post, web::{self, Data, Path}, HttpResponse, Responder
};

use bson::oid::ObjectId;
use tracing::{instrument, warn};

use crate::models::{memorize, user, MongoDatabase};

pub fn factory(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/memorize").service(get_memorize_data).service(add_memorize_data));
}

#[get("/{set_id}")]
#[instrument(skip( db,id),fields(user_id = %id.id().unwrap()))]
pub async fn get_memorize_data(
    id: Identity,
    db: Data<MongoDatabase>,
    set_id: Path<ObjectId>,
) -> impl Responder {
    let user_id = id.id().unwrap();
    let user_id = ObjectId::from_str(&user_id).unwrap();
    match memorize::get_memorize_data(&db, user_id, *set_id).await {
        Ok(Some(memorize)) => HttpResponse::Ok().json(memorize),
        Ok(None) => HttpResponse::NotFound().body("Memorize data not found"),
        Err(err) => {
            warn!("Failed to get memorize data: {}", err);
            HttpResponse::InternalServerError().body("Failed to get memorize data")
        }
    }
}

#[post("/{set_id}")]
#[instrument(skip(db, id))]
pub async fn add_memorize_data(
    id: Identity,
    db: Data<MongoDatabase>,
    set_id: Path<ObjectId>,
    answers: web::Json<Vec<memorize::model::MemorizeCardData>>,
) -> impl Responder {
    let user_id = id.id().unwrap();
    let user_id = ObjectId::from_str(&user_id).unwrap();
    match memorize::add_memorize_data(&db, user_id, *set_id, answers.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            warn!("Failed to add memorize data: {}", err);
            HttpResponse::InternalServerError().body("Failed to add memorize data")
        }
    }
}
