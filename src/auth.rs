use rocket::{
    http::{Cookie, CookieJar, SameSite},
    serde::{json::Json, Deserialize, Serialize}
};

use crate::jwt::open_jwt;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    status: String,
}

pub struct User {
    pub username: String,
    pub user_id: String,
    pub email: String,
    password: String,
    salt: String
}

fn get_user(username: &str) -> Option<User> {
    if username == "cmcderm" {
        Some(User {
            username: String::from("cmcderm"),
            user_id: String::from("fake-uuid-whatever"),
            email: String::from("connormcderm@gmail.com"),
            password: String::from("XXXFAKEPASSWORDHASHXXX"),
            salt: String::from("fakesalt")
        })
    } else {
        None
    }
}

fn get_user_by_id(user_id: &str) -> Option<User> {
    if user_id == "fake-uuid-whatever" {
        Some(User {
            username: String::from("cmcderm"),
            user_id: String::from("fake-uuid-whatever"),
            email: String::from("connormcderm@gmail.com"),
            password: String::from("XXXFAKEPASSWORDHASHXXX"),
            salt: String::from("fakesalt")
        })
    } else {
        None
    }
}

fn build_token(user: User) -> String {
    format!("fake.token.{}", user.user_id).to_string()
}

#[post("/login", format="json", data = "<req>")]
pub fn login(req: Json<LoginRequest>, jar: &CookieJar<'_>) -> Json<LoginResponse> {
    if req.username != "" && req.password != "" {
        if let Some(u) = get_user(&req.username) {
            let token = build_token(u);
            let cookie = Cookie::build(("token", token))
                .path("/")
                .secure(true)
                .same_site(SameSite::Strict);

            jar.add(cookie);

            return Json(LoginResponse {
                status: String::from("Success!"),
            });
        }
    }

    Json(LoginResponse {
        status: String::from("Failed"),
    })
}

pub fn auth(token: &str) -> Option<User> {
    // JWT token parsing
    open_jwt();
    if token == "fake.token.fake-uuid-whatever" {
        let token_id = token.split('.').last()?;

        get_user_by_id(token_id)

    } else {
        None
    }
}
