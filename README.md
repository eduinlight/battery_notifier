# Battery Notify
- Author: Eduin Garcia Cordero
- Linkedin: [https://www.linkedin.com/in/eduinlight/](https://www.linkedin.com/in/eduinlight/)
- Github: [https://github.com/eduinlight](https://github.com/eduinlight)
- Email: [eduinlight@gmail.com](mailto:eduinlight@gmail.com)
- Stack: [Rust](https://www.rust-lang.org)

## Build
```BASH
cargo build --release
```

## Use with Xmonad
Add this line to your configuration file.
```HS
    spawnOnce "sleep 5 && $PATH_TO_REPO/target/release/battery-notifier"
```
Run this command after start notification daemon. **sleep 5** is to wait 5 secons for the notification daemon to be ready.

## Config file
When the program start look for $HOME_DIRECTORY/.config/battery-notifier/config.toml.
If the config file do not exist or has some errors he use the default configuration.
```TOML
#interval in seconds to check battery level
seconds_interval = 10
[percents]
#percent to use to show a warning message
warning = 10
#percent to use to show a critical message
critical = 5
```

## License
- [MIT](./LICENSE-MIT)
- [APACHE](./LICENSE-APACHE)
