use crate::error::AppError;
use crate::model::UserConfig;

use std::fs;
use std::path::PathBuf;
use std::io;
use serde_json;

pub fn config_dir() -> Result<PathBuf, AppError> {
    let home = dirs::home_dir().ok_or(AppError::HomeNotFound)?;
    Ok(home.join(".encmsg"))
}

pub fn config_path() -> Result<PathBuf, AppError> {
    Ok(config_dir()?.join("config.json"))
}

pub fn load_or_init_config() -> Result<UserConfig, AppError> {
    let path = config_path()?;

    if path.exists() {
        let text = fs::read_to_string(&path)?;
        let cfg: UserConfig = serde_json::from_str(&text)?;
        if cfg.username.trim().is_empty() {
            return Err(AppError::MissingUsername);
        }
        return Ok(cfg);
    }

    println!("No config found, enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    let cfg = UserConfig{username};

    fs::create_dir_all(config_dir()?)?;
    fs::write(path, serde_json::to_string_pretty(&cfg)?)?;

    Ok(cfg)
}