#[macro_use] extern crate rocket;

mod auth;
mod greeting;
mod jwt;
mod database;
mod user;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/api", routes![
        index,
        auth::login,
        greeting::greet
    ])
}
