use anyhow::anyhow;
use argon2::{
    Argon2, PasswordHasher, PasswordHash, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use rand::Rng;

pub fn hash_password(password: &String) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("Password hashing failed: {}", e))?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &String, password_hash: &String) -> anyhow::Result<()> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| anyhow!("Invalid password hash format: {}", e))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| anyhow!("Invalid password"))
}

pub fn generate_refresh_token() -> anyhow::Result<String> {
    let random_bytes: [u8; 32] = rand::rng().random();
    Ok(hex::encode(random_bytes))
}

pub fn hash_token(token: &String) -> anyhow::Result<String> {
    hash_password(token)
}
