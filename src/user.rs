use tokio_postgres::Client;

use crate::database;

pub struct User {
    pub username: String,
    pub user_id: String,
    pub email: String,
}

pub enum UserCreateError {
    MissingInformation,
    UsernameExists,
    EmailExists,
    ConnectionError,
}

pub async fn create_user(username: &str, password: &str, email: &str) -> Result<User, UserCreateError> {
    if username.is_empty() || password.is_empty() || email.is_empty() {
        return Err(UserCreateError::MissingInformation);
    }

    let db_client = database::get_db_client().await;

    // Check if username or email already exists


    // Create user

    Err(UserCreateError::MissingInformation)
}

async fn username_exists(client: &Client, username: &str, email: &str) -> Result<bool, ()> {
    let row = client
        .query_opt("SELECT user_id FROM users WHERE username = $1 OR email = $2", &[&username, &email])
        .await;

    match row {
        Ok(None) => Ok(false),
        Ok(Some(_)) => Ok(true),
        Err(_) => Err(())
    }
}



