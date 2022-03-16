mod battery;
mod config;
mod notification;

use crate::battery::Battery;
use crate::config::CONFIG;
use crate::notification::send_notification;
use std::{thread, time::Duration};

fn main() {
  let mut battery = Battery::new().unwrap();
  let mut before_percent = 100;

  loop {
    battery.refresh();
    let percent = (battery.battery.state_of_charge().value * 100f32) as i16;
    if before_percent > CONFIG.percents.critical && percent <= CONFIG.percents.critical {
      send_notification(
        "Critic: Low battery level",
        "Please connect your computer to an energy provider. Or Shutdown the computer.",
        "battery",
      );
    } else if before_percent > CONFIG.percents.warning && percent <= CONFIG.percents.warning {
      send_notification(
        "Warning: Low battery level",
        "Please connect your computer to an energy provider",
        "battery",
      );
    }
    before_percent = percent;
    thread::sleep(Duration::from_secs(CONFIG.seconds_interval));
  }
}
