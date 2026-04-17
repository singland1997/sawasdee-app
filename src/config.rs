use std::env;
use dotenvy::dotenv;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .context("DATABASE_URL must be set in .env")?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .context("PORT must be a valid number")?;
        
        let jwt_secret = env::var("JWT_SECRET")
            .context("JWT_SECRET must be set")?;

        Ok(Self {
            database_url,
            port,
            jwt_secret,
        })
    }
}