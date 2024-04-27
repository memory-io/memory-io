use std::collections::HashMap;

use crate::helper::{delete_set_from_id, get_set_from_id, login_user, signup_user, spawn_app};
use memory_server::models::{
    card::Card,
    set::model::{PatchSet, Set, SetWithCards},
    user::model::UserSignup,
};
use tracing::debug;

#[tokio::test]
async fn test_set_functionality() {
    let app = spawn_app().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    // Sign up user
    let user = UserSignup {
        username: "ursula_le_guin".to_string(),
        email: "ursula_le_guin@gmail.com".to_string(),
        password: "APpleafdf".to_string(),
    };

    let response = signup_user(&client, &app.address, &user)
        .await
        .expect("Failed to signup user");
    assert_eq!(200, response.status().as_u16());

    //login in to user

    let response = login_user(&client, &app.address, &user.email, &user.password)
        .await
        .expect("Failed to login user");
    assert_eq!(200, response.status().as_u16());

    //create  aset

    let mut map = HashMap::new();
    map.insert("title", "rust");
    map.insert("visibility", "Public");

    let response = client
        .post(&format!("{}/api/sets/create", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute create set request.");

    assert_eq!(200, response.status().as_u16());

    //get sets
    let response = client
        .get(&format!("{}/api/sets", &app.address))
        .send()
        .await
        .expect("Failed to execute get sets request.");

    //check if created set is inside the sets
    assert_eq!(200, response.status().as_u16());
    let sets: Vec<Set> = response.json().await.expect("Failed to deseralize sets");
    debug!("Sets: {:?}", sets);
    let found = sets.iter().find(|a| a.title == "rust");
    assert_eq!(found.is_some(), true);
    let found = found.unwrap();

    //add card

    let response = client
        .patch(&format!("{}/api/sets/{}", &app.address, found.id.to_hex()))
        .json(&PatchSet::AddCard {
            front: "banana".to_string(),
            back: "apple".to_string(),
        })
        .send()
        .await
        .expect("Failed to execute patch set request.");
    assert_eq!(200, response.status().as_u16());

    //get set and check if card is in it
    let response = get_set_from_id(&client, &app.address, &found.id.to_hex())
        .await
        .expect("Failed to get set");
    assert_eq!(200, response.status().as_u16());
    let set = response
        .json::<SetWithCards>()
        .await
        .expect("Failed to desearlize created set");
    debug!("Set: {:?}", set);

    let card = set
        .cards
        .iter()
        .find(|a| a.front == "banana" && a.back == "apple")
        .expect("Failed to find created card");

    let response = client
        .patch(&format!("{}/api/sets/{}", &app.address, found.id.to_hex()))
        .json(&PatchSet::UpdateCard(Card {
            id: card.id.into(),
            front: "apple".to_string(),
            back: "banana".to_string(),
        }))
        .send()
        .await
        .expect("Failed to execute update card request.");
    assert_eq!(200, response.status().as_u16());

    //check if card was updated
    let response = client
        .get(&format!(
            "{}/api/sets/{}?includeCards=true",
            &app.address,
            found.id.to_hex()
        ))
        .send()
        .await
        .expect("Failed to execute get set request.");
    assert_eq!(200, response.status().as_u16());

    let set = response
        .json::<SetWithCards>()
        .await
        .expect("Failed to get the created set");

    let card = set
        .cards
        .iter()
        .find(|a| a.front == "apple" && a.back == "banana")
        .expect("Failed to find updated card");

    //delete card
    let response = client
        .patch(&format!(
            "{}/api/sets/{}?includeCards=true",
            &app.address,
            found.id.to_hex()
        ))
        .json(&PatchSet::RemoveCard { id: card.id.into() })
        .send()
        .await
        .expect("Failed to execute delete card request.");
    assert_eq!(200, response.status().as_u16());

    //check if card was deleted
    //check if card was updated
    let response = client
        .get(&format!(
            "{}/api/sets/{}?includeCards=true",
            &app.address,
            found.id.to_hex()
        ))
        .send()
        .await
        .expect("Failed to execute get set request.");
    assert_eq!(200, response.status().as_u16());

    let set = response
        .json::<SetWithCards>()
        .await
        .expect("Failed to get the created set");

    let card = set
        .cards
        .iter()
        .find(|a| a.front == a.front && a.back == a.back);
    assert_eq!(card.is_none(), true);

    //delete set
    let response = delete_set_from_id(&client, &app.address, &found.id.to_hex())
        .await
        .expect("Failed to delete set");

    assert_eq!(200, response.status().as_u16());

    //check deleted
    let response = get_set_from_id(&client, &app.address, &found.id.to_hex())
        .await
        .expect("Failed to get set");
    assert_eq!(404, response.status().as_u16());

    app.db.db().drop(None).await.unwrap();
}
