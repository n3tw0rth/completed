use crate::notification;

pub fn handle_ctrlc() {
    let _ = ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        println!("sending cancelled notification");

        notification::send_notification(
            "Process cancelled by user".to_string(),
            "Cancelled".to_string(),
        )
        .unwrap()
    });
}
