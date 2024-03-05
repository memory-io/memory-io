use crate::helper::spawn_app;
use memory_server::models::user::{User, UserSignup};
use mongodb::bson::doc;

#[tokio::test]
async fn signup_login_user() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let signup = UserSignup {
        // username: "ursula_le_guin".to_string(),
        email: "ursula_le_guin@gmail.com".to_string(),
        password: "APpleafdf".to_string(),
    };
    let response = client
        .post(&format!("{}/api/users/signup", &app.address))
        .json(&signup)
        .send()
        .await
        .expect("Failed to execute request.");

    println!("{:?}", response);
    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = app
        .db
        .db()
        .collection::<User>("users")
        .find_one(doc! {"email":&signup.email}, None)
        .await
        .unwrap()
        .expect("Failed to find user");

    assert_eq!(saved.email, signup.email);

    let response = client
        .post(&format!("{}/api/users/login", &app.address))
        .json(&signup)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    println!("{:?}", response);

    app.db.db().drop(None).await.unwrap();
}
