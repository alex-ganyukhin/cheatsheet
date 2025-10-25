#![allow(dead_code)]

use std::{env, path::PathBuf};

/// Returns the preferred configuration path for the cheat sheet data file.
pub fn default_store_path() -> PathBuf {
    env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join(".config/cheatsheet/cheatsheet.toml"))
        .unwrap_or_else(|| PathBuf::from(".cheatsheet.toml"))
}
