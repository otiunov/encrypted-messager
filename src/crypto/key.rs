use crate::error::AppError;
use x25519_dalek::{PublicKey, StaticSecret};
use std::fs;
use std::path::PathBuf;
use base64::{engine::general_purpose, Engine as _};
use zeroize::{self, Zeroize};
use serde::ser::Error;

pub struct Keypair {
    pub secret: StaticSecret,
    pub public: PublicKey
}

impl Keypair {
    pub fn new() -> Self {
        let secret = StaticSecret::random();
        let public = PublicKey::from(&secret);
        Self { secret, public }
    }
}

fn key_path(key_type: &str) -> Result<PathBuf, AppError> {
    Ok(crate::config::config_dir()?.join(key_type))
}

pub fn private_key_path() -> Result<PathBuf, AppError> {
    key_path("private.key")
}

pub fn public_key_path() -> Result<PathBuf, AppError> {
    key_path("public.key")
}

pub fn save_keypair(keypair: &Keypair) -> Result<(), AppError> {
    fs::write(private_key_path()?, keypair.secret.to_bytes())?;

    let encoded_pub_key = general_purpose::STANDARD.encode(keypair.public.as_bytes());
    fs::write(public_key_path()?, encoded_pub_key)?;

    Ok(())
}

pub fn load_keypair() -> Result<Keypair, AppError> {
    fs::create_dir_all(crate::config::config_dir()?)?;
    let secret_bytes = fs::read(private_key_path()?)?;
    if secret_bytes.len() != 32 {
        // define a new variant if you like
        return Err(AppError::ConfigParse(
            serde_json::Error::custom("Invalid private key length")
        ));
    }

    let public_b64 = fs::read_to_string(public_key_path()?)?;

    let mut secret_arr = [0u8; 32];
    secret_arr.copy_from_slice(&secret_bytes[0..32]);
    let secret = StaticSecret::from(secret_arr);
    secret_arr.zeroize();

    let public_bytes = general_purpose::STANDARD.decode(public_b64.trim())?;
    let mut public_arr = [0u8; 32];
    public_arr.copy_from_slice(&public_bytes[0..32]);
    let public = PublicKey::from(public_arr);

    Ok(Keypair { secret, public })
}

pub fn load_or_generate_keypair() -> Result<Keypair, AppError> {
    if private_key_path()?.exists() && public_key_path()?.exists() {
        return load_keypair();
    }

    let keypair = Keypair::new();
    save_keypair(&keypair)?;
    Ok(keypair)
}

pub fn format_pub_key(keypair: &Keypair) -> String {
    general_purpose::STANDARD.encode(keypair.public.as_bytes())
}