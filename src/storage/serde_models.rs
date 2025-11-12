//! The file contains definitions of domain entries as serde-serializable models.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TomlEntry {
    pub title: String,
    pub command: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TomlEntryStorageDef {
    pub entry: Vec<TomlEntry>,
}
