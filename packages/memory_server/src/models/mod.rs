use mongodb::{Client, Database};

pub mod card;
pub mod set;
pub mod user;
#[derive(Clone)]
pub struct MongoDatabase {
    pub client: Client,
    pub database: Database,
}

impl MongoDatabase {
    pub fn new(client: Client, database: &str) -> MongoDatabase {
        let db = client.database(&database);
        MongoDatabase {
            client,
            database: db,
        }
    }
    pub fn db(&self) -> mongodb::Database {
        self.database.clone()
    }
}
