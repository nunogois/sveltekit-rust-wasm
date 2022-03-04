mod utils;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, WebAssembly!");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Joke {
    pub setup: String,
    pub delivery: String,
}

#[wasm_bindgen]
pub async fn joke() -> Result<JsValue, JsValue> {
    let url = "https://v2.jokeapi.dev/joke/programming,miscellaneous,pun,spooky,christmas?blacklistFlags=nsfw,religious,political,racist,sexist,explicit&type=twopart";
    let joke = reqwest::get(url)
        .await
        .unwrap()
        .json::<Joke>()
        .await
        .unwrap();
    Ok(JsValue::from_serde(&joke).unwrap())
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[wasm_bindgen]
impl User {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, email: String) -> User {
        User {
            id: Uuid::new_v4().to_string(),
            name,
            email,
        }
    }
}

#[wasm_bindgen]
pub async fn users_load() -> Result<JsValue, JsError> {
    let url = "http://localhost:8080/users";
    match reqwest::get(url).await {
        Ok(response) => {
            let users: Vec<User> = response.json::<Vec<User>>().await.unwrap();
            Ok(JsValue::from_serde(&users).unwrap())
        }
        Err(_e) => Err(JsError::new(
            "Something went wrong trying to load the users. Make sure rust-actix is running.",
        )),
    }
}

#[wasm_bindgen]
pub async fn users_new(user: User) -> Result<JsValue, JsValue> {
    let url = "http://localhost:8080/users";
    let client = reqwest::Client::new();
    let new_user = client
        .post(url)
        .json(&user)
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();
    Ok(JsValue::from_serde(&new_user).unwrap())
}

#[wasm_bindgen]
pub async fn users_update(id: String, mut user: User) -> Result<JsValue, JsValue> {
    let url = format!("http://localhost:8080/users/{}", id);
    let client = reqwest::Client::new();
    user.id = id;
    let new_user = client
        .put(url)
        .json(&user)
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();
    Ok(JsValue::from_serde(&new_user).unwrap())
}

#[wasm_bindgen]
pub async fn users_delete(id: String) -> Result<String, JsValue> {
    let url = format!("http://localhost:8080/users/{}", id);
    let client = reqwest::Client::new();
    let res = client
        .delete(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    Ok(res)
}
