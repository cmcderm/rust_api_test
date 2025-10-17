#[macro_use] extern crate rocket;

mod auth;
mod greeting;
mod jwt;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![
        index,
        auth::login,
        greeting::greet
    ])
}
