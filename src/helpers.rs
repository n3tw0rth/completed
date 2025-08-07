use anyhow::{Context, Result};
use std::env;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::Config;

/// Load the app configuration, or create a default one if missing.
///
/// |Platform | Configuration file                                  |
/// | ------- | --------------------------------------------------- |
/// | Linux   | ~/.config/<pkg_name>/config.toml                    |
/// | Windows | C:\Users\<User>\AppData\Roaming\<pkg_name>\...     |
pub async fn get_app_config() -> Result<Config> {
    const FILENAME: &str = "config.toml";
    const PKG_NAME: &str = env!("CARGO_PKG_NAME");

    let config_path = dirs::config_dir()
        .context("Unable to locate user config directory")?
        .join(PKG_NAME);
    let file_path = config_path.join(FILENAME);

    if !fs::try_exists(&file_path).await? {
        fs::create_dir_all(&config_path).await?;
        let mut file = File::create(&file_path).await?;
        file.write_all(super::constants::CONFIG_FILE_CONTENT.as_bytes())
            .await?;
        println!("Created default config at {:?}", file_path);
    }

    let mut file = File::open(&file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let config: Config = toml::from_str(&contents).context("Failed to parse TOML config")?;

    Ok(config)
}
