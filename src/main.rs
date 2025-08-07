use clap::Parser;
use completed::{helpers, notification, Args};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments
    let mut args = Args::parse();

    let config = helpers::get_app_config().await?;

    let program = args.run.get(0).cloned().unwrap();
    let program_args = args.run.split_off(1);

    // Spawn the subprocess and pipe its stdout
    let mut child = Command::new(program)
        .args(program_args)
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    // Take the stdout
    let stdout = child.stdout.take().expect("Failed to capture stdout");

    // Wrap stdout in a BufReader and read it line by line
    let mut reader = BufReader::new(stdout).lines();

    while let Some(line) = reader.next_line().await? {
        // Stdio::piped() does not write to parent stdout
        // following line will handle it
        println!("{}", line);

        if args.triggers.is_some() {
            // Check if the line contains a trigger string
            let contained_triggers: Vec<_> = args
                .triggers
                .as_ref()
                .unwrap()
                .split(",")
                .filter(|trigger| line.contains(trigger))
                .collect();

            // send notification on each trigger
            if contained_triggers.len() > 0 {
                notification::Notification::new(
                    &config,
                    &args.profiles.as_ref().unwrap(),
                    "Trigger Detected".to_string(),
                    contained_triggers.join(","),
                )
                .send_trigger()
                .await
                .unwrap();
            }
        }
    }

    let status = child.wait().await.expect("failed to wait on child");
    let (title, msg) = match status.code() {
        Some(code) => {
            // Handle exit codes
            match code {
                0 => ("Process completed".to_string(), "Success".to_string()),
                1 => ("Process errored".to_string(), "Error".to_string()),
                _ => ("Something went wrong".to_string(), "Unknown".to_string()),
            }
        }
        None => (
            "Process terminated with unknown reason".to_string(),
            "Unknown".to_string(),
        ),
    };

    // Send the notication for status code
    notification::Notification::new(&config, &args.profiles.unwrap(), title, msg)
        .send()
        .await?;

    Ok(())
}
