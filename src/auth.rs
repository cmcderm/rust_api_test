use rocket::{http::{Cookie, CookieJar, SameSite}, serde::{json::Json, Deserialize, Serialize}};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    status: String,
}

struct User {
    username: String,
    user_id: String,
    email: String,
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
