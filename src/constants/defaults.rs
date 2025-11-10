use std::sync::LazyLock;

pub const CHEATSHEET_OPERATING_DIR: &str = "~/.config/cheatsheet/";
pub const CONFIG_FILE_NAME: &str = "cheatsheet.toml";
pub static CONFIG_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}{}", CHEATSHEET_OPERATING_DIR, CONFIG_FILE_NAME));
