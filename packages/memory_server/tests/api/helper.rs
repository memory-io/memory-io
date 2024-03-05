use std::{collections::HashMap, net::TcpListener};

use memory_server::{
    models::{
        user::{User, UserSignup},
        MongoDatabase,
    },
    startup::{initialize_db, run, ServerConfig},
};
use mongodb::{bson::doc, options::ClientOptions, Client};
use reqwest::Response;


pub struct TestApp {
    pub address: String,
    pub db: MongoDatabase,
}
impl Drop for TestApp {
    fn drop(&mut self) {
        // let db = self.db.clone();
        // tokio::task::spawn(async move {
        //     db.db().drop(None).await.unwrap();
        // });
    }
}
pub async fn spawn_app() -> TestApp {
    env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("debug")).ok();

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let database_name = format!("test-{port}");

    let options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let db = Client::with_options(options).unwrap();
    let mongo_database = MongoDatabase::new(db, &database_name);
    initialize_db(&mongo_database).await.unwrap();

    let server = run(
        mongo_database.clone(),
        listener,
        ServerConfig { test_mode: true },
    )
    .await
    .expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db: mongo_database,
    }
}


pub async fn get_set_from_id(client: &reqwest::Client, address: &String, id: &str) -> Result<Response, reqwest::Error> {
    let response = client
        .get(&format!("{}/api/sets/{}", address, id))
        .send()
        .await;
    
    response
}

pub async fn delete_set_from_id(client: &reqwest::Client, address: &String, id: &str) -> Result<Response, reqwest::Error> {
    let response = client
        .delete(&format!("{}/api/sets/{}", address, id))
        .send()
        .await;
    
    response
}


pub async fn signup_user(client: &reqwest::Client, address: &String, user: &UserSignup) -> Result<Response, reqwest::Error> {
  
    let response = client
        .post(&format!("{}/api/users/signup", address))
        .json(&user)
        .send()
        .await;
    
    response
}

pub async fn login_user(client: &reqwest::Client, address: &String, email: &String,password:&String) -> Result<Response, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("email", email);
    map.insert("password", password);

    let response = client
        .post(&format!("{}/api/users/login", address))
        .json(&map)
        .send()
        .await;
    
    response
}