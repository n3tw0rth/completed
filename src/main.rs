use std::collections::HashMap;
use std::{env, process::Command};

use clap::Parser;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use toml::de::from_str;

mod constants;
mod helpers;
mod notification;
mod send_emails;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None,arg_required_else_help = true,trailing_var_arg=true)]
#[clap(
    about = constants::ABOUT_TEXT,
)]
struct Args {
    #[clap(required = true)]
    pub run: Vec<String>,

    #[arg(short, long)]
    #[clap(default_value = "default")]
    profiles: Option<Vec<String>>,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    triggers: Option<Vec<String>>,

    #[arg(long, short, action)]
    verbose: bool,
}

#[derive(Deserialize, Debug)]
struct Config {
    email: HashMap<String, EmailConfig>,
    gchat: Option<HashMap<String, GChatConfig>>,
    profiles: HashMap<String, ProfileConfig>,
}
#[derive(Deserialize, Debug)]
struct ProfileConfig {
    print_output: Option<bool>,
    sendto: Vec<String>,
}
#[derive(Deserialize, Debug)]
struct GChatConfig {
    webhook: String,
    api_key: String,
}
#[derive(Deserialize, Debug, Clone)]
struct EmailConfig {
    from: String,
    to: String,
    username: String,
    password: String,
    port: u16,
    host: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", std::env::var("CARGO_PKG_NAME").unwrap());
    let mut args = Args::parse();

    let config = helpers::app_state().await?;

    // helpers::handle_ctrlc().await;

    let program = args.run.get(0).cloned().unwrap();
    let program_args = args.run.split_off(1);

    let mut child = Command::new(program)
        .args(program_args)
        .spawn()
        .expect("failed to execute child");

    let status = child.wait().expect("failed to wait on child");
    let (title, msg) = match status.code() {
        Some(code) => {
            // should hand the exit code type, for now consider only is 0
            println!("Exited with status code: {code}");

            match code {
                0 => ("Process completed".to_string(), "Success".to_string()),
                1 => ("Process errored".to_string(), "Error".to_string()),
                _ => ("Something went wrong".to_string(), "Unknown".to_string()),
            }
        }
        None => (
            "Process terminated by signal".to_string(),
            "Cancelled".to_string(),
        ),
    };

    let _ = notification::Notification::new(&config, &args.profiles.unwrap(), title, msg)
        .send()
        .await;

    Ok(())
}
