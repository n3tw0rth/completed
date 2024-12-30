use std::process::Command;

pub fn send_notification(title: String, msg: String) -> anyhow::Result<()> {
    let output = Command::new("gdbus")
        .arg("call")
        .arg("-e")
        .arg("-d")
        .arg("org.freedesktop.Notifications")
        .arg("-o")
        .arg("/org/freedesktop/Notifications")
        .arg("-m")
        .arg("org.freedesktop.Notifications.Notify")
        .arg("app name")
        .arg("0")
        .arg("icon")
        .arg(title)
        .arg(msg)
        .arg("[]")
        .arg("{'urgency': <byte 1>}")
        .arg("3000")
        .output() // Run the command and collect the output
        .expect("Failed to execute command");

    Ok(())
}
