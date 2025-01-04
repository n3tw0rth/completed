use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

//pub async fn handle_ctrlc() {
//    let _ = ctrlc::set_handler(move || {
//        println!("received Ctrl+C!");
//        println!("sending cancelled notification");
//    });
//}
use super::Config;

pub async fn app_state() -> anyhow::Result<Config> {
    //let path = dirs::config_dir().unwrap();

    // Asynchronously open the TOML file
    let mut file = File::open("config.toml").await?;

    // Read the file content into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    // Parse the TOML content
    let config: Config = toml::from_str(&contents).expect("Failed to parse TOML");

    Ok(config)
}
