extern crate battery;
use battery::Error;
use battery::{Battery as B, Manager};
use std::io;

pub struct Battery {
  pub battery: B,
  manager: Manager,
}

impl Battery {
  pub fn new() -> Result<Self, Error> {
    let manager = battery::Manager::new()?;
    let battery = match manager.batteries()?.next() {
      Some(Ok(battery)) => battery,
      Some(Err(e)) => {
        eprintln!("Unable to access battery information");
        return Err(e);
      }
      None => {
        eprintln!("Unable to find any batteries");
        return Err(io::Error::from(io::ErrorKind::NotFound).into());
      }
    };
    Ok(Battery { manager, battery })
  }

  pub fn refresh(&mut self) {
    self.manager.refresh(&mut self.battery).unwrap();
  }
}
