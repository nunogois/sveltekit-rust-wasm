mod utils;

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

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

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
}

#[wasm_bindgen]
impl User {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, email: String) -> User {
        User { name, email }
    }
}

#[wasm_bindgen]
pub async fn get_users() -> Result<JsValue, JsValue> {
    let url = "http://localhost:8080/users";
    let users = reqwest::get(url)
        .await
        .unwrap()
        .json::<Vec<User>>()
        .await
        .unwrap();
    Ok(JsValue::from_serde(&users).unwrap())
}

#[wasm_bindgen]
pub async fn post_user(user: User) -> Result<JsValue, JsValue> {
    println!("post_user");
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
