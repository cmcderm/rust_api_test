use rocket::{http::{CookieJar, Status}, response::status};

use crate::auth::auth;

#[get("/greet")]
pub fn greet(jar: &CookieJar<'_>) -> Result<String, status::Unauthorized<String>>{
    if let Some(token) = jar.get("token") {
        if let Some(user) = auth(token.value_trimmed()) {
            Ok(format!("Hello, {}", &user.username))
        } else {
            Err(status::Unauthorized(String::from("Your token is no good")))
        }
    } else {
        Err(status::Unauthorized(String::from("I dunno who you are")))
    }
}
