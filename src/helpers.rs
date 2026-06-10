use anyhow::{Context, Result};
use std::env;
use tokio::fs::{self};

use super::Config;

/// Load the app configuration, or create a default one if missing.
pub async fn get_app_config() -> Result<Config> {
    const FILENAME: &str = "config.toml";
    const PKG_NAME: &str = env!("CARGO_PKG_NAME");

    let file_path = dirs::config_dir()
        .context("Unable to locate user config directory")?
        .join(PKG_NAME)
        .join(FILENAME);

    if !fs::try_exists(&file_path).await? {
        fs::create_dir_all(file_path.parent().unwrap()).await?;
        fs::write(&file_path, super::constants::CONFIG_FILE_CONTENT).await?;
        println!("Created default config at {:?}", file_path);
    }

    let contents = fs::read_to_string(file_path).await?;

    toml::from_str(&contents).context("Failed to parse TOML config")
}
