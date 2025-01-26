mod config;
mod controller;
mod db;
mod dto;
mod entity;
mod routes;
mod service;
mod utils;

use crate::routes::init_routes;
use actix_web::{web, App, HttpServer};
use config::Config;
use db::establish_connection;
use migration::{Migrator, MigratorTrait};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    // DATABASE CONNECTION
    let db = establish_connection(&config.database_url).await;

    // APPLY MIGRATIONS
    Migrator::up(&db, None).await.unwrap();

    // APP SERVER
    HttpServer::new(move || {
        let db = db.clone();
        App::new()
            .app_data(web::Data::new(db))
            .configure(init_routes)
    })
    .bind((config.url, config.port))?
    .run()
    .await
}
