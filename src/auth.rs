use rocket::{
    http::{Cookie, CookieJar, SameSite}, response::status, serde::{json::Json, Deserialize, Serialize}
};
use tokio_postgres::Client;

use crate::{
    database,
    jwt::{self, open_jwt}, user::{self, UserCreateError}
};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    status: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    username: String,
    password: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    status: String,
    message: String,
    username: String,
    email: String,
}

pub struct UserLogin {
    pub user: user::User;
    pub password_hash: String,
    pub salt: String,
}

// Handle pulling the authentication cookie and returning user info
// Allows callers of this function to assume the user is fully authenticated
// As long as the User is held
pub async fn auth(jar: &CookieJar<'_>) -> Option<User> {
    let mut authorization = jar.get("Authorization")?.value().split(" ");

    // Expecting format of "Bearer <token>"
    if authorization.next() != Some("Bearer") {
        return None;
    }

    let token = authorization.next()?;

    let opened_jwt = open_jwt(token)?;

    let client = database::get_db_client().await.ok()?;

    get_user_by_id(&client, &opened_jwt.payload.sub).await.ok()
}

// Logs a user in, tokens are not stored server side
// For better security, tokens should be saved to allow for revocation
#[post("/login", format="json", data = "<req>")]
pub async fn login(req: Json<LoginRequest>, jar: &CookieJar<'_>) -> Result<Json<LoginResponse>, ()> {
    // Don't bother for empty login
    if req.username == "" && req.password == "" {
        return Ok(Json(LoginResponse {
            status: String::from("Failed"),
        }));
    }

    let client = database::get_db_client().await.map_err(|_| ())?;
    // Retrieve login information from database
    let user_login = match get_user_login_by_username(&client, &req.username).await {
        Ok(u) => u,
        Err(_) => return
            Ok(Json(LoginResponse {
                status: String::from("Failed"),
            })),
    };

    // Verify given login against database info
    if !verify_login(&user_login, &req.password).await? {
        return Ok(Json(LoginResponse {
            status: String::from("Failed"),
        }));
    }

    // Create JWT and assign to Auth cookie
    let token = jwt::create_jwt(user_login);

    let cookie = Cookie::build(("Authorization", format!("Bearer {}", token)))
        .path("/")
        .secure(true)
        .same_site(SameSite::Strict);

    jar.add(cookie);

    return Ok(Json(LoginResponse {
        status: String::from("Success!"),
    }));
}

#[post("/create_user", format="json", data = "<req>")]
pub async fn create_user(req: Json<CreateUserRequest>) -> Result<Json<CreateUserResponse>, status::BadRequest<String>> {
    match user::create_user(&req.username, &req.password, &req.email).await {
        Ok(u) => Ok(Json(CreateUserResponse {
            status: String::from("Success"),
            message: String::from("User created successfully"),
            username: u.username.clone(),
            email: u.email.clone(),
        })),
        Err(e) => match e {
            UserCreateError::MissingInformation => Err(status::BadRequest(String::from("Missing required information."))),
            UserCreateError::UsernameExists => Err(status::BadRequest(String::from("Username already taken."))),
            UserCreateError::EmailExists => Err(status::BadRequest(String::from("Email already in use."))),
        }
    }
}

async fn get_user_login_by_username(client: &Client, username: &str) -> Result<UserLogin, ()> {
    if let Ok(user_info) = database::get_user_login_by_username(client, username).await {
        return Ok(UserLogin {
            user: user::User {
                user_id: user_info.0,
                username: user_info.1,
                email: String::from(""),
            },
            password_hash: user_info.2,
            salt: user_info.3,
        });
    } else {
        return Err(());
    }
}

async fn get_user_by_id(client: &Client, user_id: &str) -> Result<user::User, ()> {
    if let Ok(user) = database::get_user_by_id(client, user_id).await {
        return Ok(user::User {
            user_id: user.0,
            username: user.1,
            email: user.2,
        })
    } else {
        return Err(());
    }
}

async fn verify_login(user_login: &UserLogin, given_password: &str) -> Result<bool, ()> {
    let salted_password = format!("{}{}", given_password, user_login.salt);

    match bcrypt::verify(&salted_password, &user_login.password_hash) {
        Ok(valid) => Ok(valid),
        Err(_) => Err(()),
    }
}

