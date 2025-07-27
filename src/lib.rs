use std::collections::HashMap;

use clap::Parser;
use serde::Deserialize;

pub mod constants;
pub mod enums;
pub mod helpers;
pub mod notification;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None,arg_required_else_help = true,trailing_var_arg=true)]
#[clap(
    about = constants::ABOUT_TEXT,
)]
pub struct Args {
    #[clap(required = true)]
    pub run: Vec<String>,

    #[arg(short, long)]
    #[clap(default_value = "default")]
    pub profiles: Option<Vec<String>>,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    pub triggers: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    email: Option<HashMap<String, EmailConfig>>,
    gchat: Option<HashMap<String, GChatConfig>>,
    profiles: HashMap<String, ProfileConfig>,
}
#[derive(Deserialize, Debug)]
struct ProfileConfig {
    sendto: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct GChatConfig {
    webhook: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct EmailConfig {
    from: String,
    to: String,
    username: String,
    password: String,
    port: u16,
    host: String,
}
