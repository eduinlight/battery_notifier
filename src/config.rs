use config::{Config as Settings, ConfigError, File};
use home::home_dir;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Percents {
  pub warning: i16,
  pub critical: i16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Config {
  pub percents: Percents,
  pub seconds_interval: u64,
}

fn get_home_dir() -> String {
  String::from(home_dir().unwrap().to_str().unwrap())
}

fn get_config_file_path() -> String {
  format!("{}/.config/battery_notifier/config.toml", get_home_dir())
}

impl Config {
  pub fn new() -> Result<Self, ConfigError> {
    let s = Settings::builder()
      .add_source(File::with_name(get_config_file_path().as_str()))
      .build()?;

    s.try_deserialize()
  }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| match Config::new() {
  Ok(c) => c,
  Err(_) => {
    eprintln!(
      "Config file not found or contain errors {}",
      get_config_file_path()
    );
    Config {
      percents: Percents {
        warning: 10,
        critical: 5,
      },
      seconds_interval: 10,
    }
  }
});
