use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::Config;

pub async fn app_state() -> anyhow::Result<Config> {
    let config_path = dirs::config_dir().unwrap().join("completion-notifier");
    let filename = "config.toml";

    let path = config_path.join(filename);

    println!("{:?}", path);
    if !path.exists() {
        // Ensure the parent directory exists
        tokio::fs::create_dir_all(&config_path).await?;

        // Create the file and write some initial content
        let mut file = File::create(&path).await?;
        file.write_all(super::constants::CONFIG_FILE_CONTENT.as_bytes())
            .await?;

        println!("File created: {:?}", path);
    }

    // Asynchronously open the TOML file
    let mut file = File::open(path).await?;

    // Read the file content into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    // Parse the TOML content
    let config: Config = toml::from_str(&contents).expect("Failed to parse TOML");

    Ok(config)
}
