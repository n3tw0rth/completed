use anyhow::Context;
use clap::Parser;
use hark::{helpers, notification::Notification, Args, Config};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

/// Send a notification for a line that contains one of the trigger strings,
/// printing a warning instead of failing when a destination is unreachable
async fn check_triggers(line: &str, args: &Args, config: &Config) {
    let Some(triggers) = &args.triggers else {
        return;
    };

    let contained_triggers: Vec<&str> = triggers
        .iter()
        .map(String::as_str)
        .filter(|trigger| line.contains(trigger))
        .collect();

    if contained_triggers.is_empty() {
        return;
    }

    if let Err(err) = Notification::new(
        config,
        &args.profiles,
        args.name.as_deref(),
        "Trigger Detected".to_string(),
        contained_triggers.join(","),
    )
    .send_trigger()
    .await
    {
        eprintln!("hark: failed to send trigger notification: {err}");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments
    let mut args = Args::parse();

    let config = helpers::get_app_config().await?;

    // clap guarantees at least one value for `run`
    let program = args.run.first().cloned().unwrap();
    let program_args = args.run.split_off(1);

    // Spawn the subprocess and pipe its stdout and stderr
    let mut child = Command::new(&program)
        .args(program_args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to spawn '{program}'"))?;

    let stdout = child.stdout.take().context("failed to capture stdout")?;
    let stderr = child.stderr.take().context("failed to capture stderr")?;

    let mut stdout_lines = BufReader::new(stdout).lines();
    let mut stderr_lines = BufReader::new(stderr).lines();

    let mut stdout_done = false;
    let mut stderr_done = false;

    // Read both streams until they close, echoing each line to the matching
    // parent stream (Stdio::piped() does not write to the terminal) and
    // checking it for trigger strings
    while !(stdout_done && stderr_done) {
        let line = tokio::select! {
            line = stdout_lines.next_line(), if !stdout_done => match line? {
                Some(line) => {
                    println!("{line}");
                    Some(line)
                }
                None => {
                    stdout_done = true;
                    None
                }
            },
            line = stderr_lines.next_line(), if !stderr_done => match line? {
                Some(line) => {
                    eprintln!("{line}");
                    Some(line)
                }
                None => {
                    stderr_done = true;
                    None
                }
            },
        };

        if let Some(line) = line {
            check_triggers(&line, &args, &config).await;
        }
    }

    let status = child.wait().await.context("failed to wait on child")?;
    let (title, msg) = match status.code() {
        Some(0) => ("Process completed".to_string(), "Success".to_string()),
        Some(code) => (
            "Process errored".to_string(),
            format!("Exited with code {code}"),
        ),
        None => (
            "Process terminated".to_string(),
            "Killed by a signal".to_string(),
        ),
    };

    // Send the notification for the exit status; a failed notification should
    // not mask the child's exit code
    if let Err(err) = Notification::new(&config, &args.profiles, args.name.as_deref(), title, msg)
        .send()
        .await
    {
        eprintln!("hark: failed to send notification: {err}");
    }

    // Propagate the child's exit code to the caller
    std::process::exit(status.code().unwrap_or(1));
}
