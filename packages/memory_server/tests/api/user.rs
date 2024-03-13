use crate::helper::{get_user, login_user, signup_user, spawn_app};
use memory_server::models::user::{User, UserSendable, UserSignup};
use mongodb::bson::doc;

#[tokio::test]
async fn signup_login_user() -> anyhow::Result<()> {
    let app = spawn_app().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    // Act
    let user = UserSignup {
        username: "ursula_le_guin".to_string(),
        email: "ursula_le_guin@gmail.com".to_string(),
        password: "APpleafdf".to_string(),
    };
    let response = signup_user(&client, &app.address, &user).await?;
    assert_eq!(response.status(), 200);

    let response = login_user(&client, &app.address, &user.email, "Not the passsword").await?;
    assert_eq!(response.status(), 401);

    let response = login_user(&client, &app.address, &user.email, &user.password).await?;
    assert_eq!(response.status(), 200);

    let response = get_user(&client, &app.address).await?;
    assert_eq!(response.status(), 200);

    let recieved_user: UserSendable = response.json().await?;
    assert_eq!(recieved_user.email, user.email);

    app.db.db().drop(None).await.unwrap();
    Ok(())
}
