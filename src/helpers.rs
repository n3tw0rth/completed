use std::env;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::Config;

/// Get the app configuration on runtime
/// if the configuration file is missing a new file will be created
///
/// |Platform | Configuration file                                               |
/// | ------- | ---------------------------------------------------- |
/// | Linux   | /home/alice/.config/completed/config.toml            |
/// | Windows | C:\Users\Alice\AppData\Roaming\completed\config.toml |
///
pub async fn get_app_config() -> anyhow::Result<Config> {
    let filename = "config.toml";
    let pkg_name = env::var("CARGO_PKG_NAME")
        .expect("Package name is missing, there is nothing you can do about it :) ");

    let config_path = dirs::config_dir().unwrap().join(&pkg_name);
    let path = config_path.join(filename);

    if !path.exists() {
        // Ensure the parent directory exists
        tokio::fs::create_dir_all(&config_path).await?;

        // Create the file and write some initial content
        let mut file = File::create(&path).await?;
        file.write_all(super::constants::CONFIG_FILE_CONTENT.as_bytes())
            .await?;

        println!("File created: {:?}", path);
    }

    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    // Parse the TOML content
    let config: Config = toml::from_str(&contents).expect("Failed to parse TOML");

    Ok(config)
}
