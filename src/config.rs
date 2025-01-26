use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub url: String,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();

        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE URL MUST BE SET"),
            port: env::var("PORT")
                .unwrap_or("8080".to_string())
                .parse()
                .expect("PORT MUST BE A NUMBER"),
            url: env::var("URL").expect("URL MUST BE SET"),
        }
    }
}
