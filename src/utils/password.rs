use anyhow::Result;
use anyhow::{Context, anyhow};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use tokio::task::spawn_blocking;

pub async fn hash_password(password: &str) -> Result<String> {
    let password_owned = password.to_string();

    spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password_owned.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| anyhow!("Failed to hash password: {}", e))
    })
    .await
    .context("Task panicked while hashing password")?
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let password_owned = password.to_string();
    let hash_owned = hash.to_string();

    spawn_blocking(move || {
        let parsed_hash =
            PasswordHash::new(&hash_owned).map_err(|e| anyhow!("Failed to parse hash: {}", e))?;

        Argon2::default()
            .verify_password(password_owned.as_bytes(), &parsed_hash)
            .map(|_| true)
            .map_err(|_| anyhow!("Invalid password"))
    })
    .await
    .context("Task panicked while verifying password")?
}
