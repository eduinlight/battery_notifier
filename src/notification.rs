use notify_rust::Notification;

pub fn send_notification(summary: &str, body: &str, icon: &str) {
  match Notification::new()
    .summary(summary)
    .icon(icon)
    .body(body)
    .show()
  {
    Ok(_) => {}
    Err(e) => println!("{:?}", e),
  }
}
