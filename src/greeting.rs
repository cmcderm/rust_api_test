use rocket::{http::{CookieJar}, response::status};

use crate::auth;

#[get("/greet")]
pub async fn greet(jar: &CookieJar<'_>) -> Result<String, status::Unauthorized<String>>{
    match auth::auth(jar).await {
        Some(user) =>  Ok(format!("Hello, {}", &user.username)),
        None => Err(status::Unauthorized(String::from("You are not logged in")))
    }
}

