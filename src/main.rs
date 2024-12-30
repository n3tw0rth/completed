use dbus::blocking::Connection;
use std::thread;
use std::time::Duration;
use std::{env, process::Command};

use clap::Parser;

mod helpers;
mod notification;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None,trailing_var_arg=true)]
struct Args {
    pub run: Vec<String>,

    #[arg(short, long)]
    profile: String,

    #[arg(long, short, action)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();

    helpers::handle_ctrlc();

    let _ = notification::send_notification(
        "Process Started".to_string(),
        "Will let you know when this process completes".to_string(),
    )
    .unwrap();

    let program = args.run.get(0).cloned().unwrap();
    let program_args = args.run.split_off(1);

    let mut child = Command::new(program)
        .args(program_args)
        .spawn()
        .expect("failed to execute child");

    let status = child.wait().expect("failed to wait on child");

    match status.code() {
        Some(code) => {
            // should hand the exit code type, for now consider only is 0
            println!("Exited with status code: {code}");

            match code {
                0 => {
                    notification::send_notification(
                        "Process completed".to_string(),
                        "Success".to_string(),
                    )
                    .unwrap();
                }
                1 => {
                    notification::send_notification(
                        "Process errored".to_string(),
                        "Error".to_string(),
                    )
                    .unwrap();
                }
                _ => {
                    notification::send_notification(
                        "Something went wrong".to_string(),
                        "Panic".to_string(),
                    )
                    .unwrap();
                }
            }
        }
        None => println!("Process terminated by signal"),
    }

    Ok(())
}
