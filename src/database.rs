use tokio_postgres::{NoTls, Client, Error};

pub fn get_conn_str() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub async fn get_db_client() -> Result<Client, Error> {
    let (client, connection) =
        tokio_postgres::connect(&get_conn_str(), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

// Move into users.rs later?
pub async fn get_user_login_by_username(client: &Client, username: &str) -> Result<(String, String, String, String), Error> {
    // TODO: Change to stored procedures
    let row = client
        .query_one("SELECT user_id, username, password_hash, salt FROM users WHERE username = $1", &[&username])
        .await?;

    let user_id: String = row.get(0);
    let username: String = row.get(1);
    let password_hash: String = row.get(2);
    let salt: String = row.get(3);

    Ok((user_id, username, password_hash, salt))
}

pub async fn get_user_by_id(client: &Client, user_id: &str) -> Result<(String, String, String), Error> {
    let row = client
        .query_one("SELECT user_id, username, email FROM users WHERE user_id = $1", &[&user_id])
        .await?;

    let user_id: String = row.get(0);
    let username: String = row.get(1);
    let email: String = row.get(2);

    Ok((user_id, username, email))
}
