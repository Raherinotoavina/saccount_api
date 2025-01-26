use sea_orm::ConnectOptions;
use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection(database_url: &str) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(database_url);
    opt.sqlx_logging(true);

    Database::connect(opt)
        .await
        .expect("Failed to connect to the database")
}
