use crate::helper::spawn_app;



#[tokio::test]
async fn signup_login_user() -> anyhow::Result<()> {
    let app = spawn_app().await;
    let _client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    

    app.db.db().drop(None).await.unwrap();
    Ok(())
}
