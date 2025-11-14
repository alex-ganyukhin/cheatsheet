use std::fs;
use std::path::{Path, PathBuf};


use anyhow::Context;
use strum_macros::{AsRefStr, Display};


use crate::storage::serde_models::TomlEntryStorageDef;


use crate::domain::{Entry, EntryStorage};


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


#[derive(Debug, AsRefStr, Display)]


pub enum TomlEntryStorageError {
    /// Used when no more specific error variant is applicable.
    #[strum(to_string = "GenericError {0}")]
    Generic(String),

    /// Used to indicate that operation is failed due to unknown/unrecognized format
    #[strum(to_string = "InvalidFormat {0}")]
    InvalidFormat(String),

    /// Used when the file cannot be opened due to any error
    #[strum(to_string = "CannotOpen {0}")]
    CannotOpen(String),
}


impl std::error::Error for TomlEntryStorageError {}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


/// File-based EntryStorage.


pub struct TomlEntryStorage {
    path: PathBuf,
}


impl TomlEntryStorage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }


    pub fn path(&self) -> &Path {
        &self.path
    }
}


impl EntryStorage for TomlEntryStorage {
    fn load_all(&self) -> Result<Vec<Entry>, anyhow::Error> {
        let loaded_toml_string: String = fs::read_to_string(&self.path).with_context(|| {
            TomlEntryStorageError::CannotOpen(format!("Cannot open file at path {}", self.path.display()))
        })?;


        let parsed_entries: TomlEntryStorageDef = toml::from_str(&loaded_toml_string).with_context(|| {
            TomlEntryStorageError::InvalidFormat(format!(
                "Cannot parse TOML content from file at path {}",
                self.path.display()
            ))
        })?;


        parsed_entries
            .entry
            .into_iter()
            .map(|e| {
                Ok(Entry {
                    title:       e.title,
                    command:     e.command,
                    description: e.description,
                })
            })
            .collect()
    }


    fn add_entry(&self, _entry: Entry) -> Result<(), anyhow::Error> {
        return Err(anyhow::anyhow!("FileEntryStorage.add_entry is not implemented yet"));
    }


    fn save_all(&self, _entries: &[Entry]) -> Result<(), anyhow::Error> {
        return Err(anyhow::anyhow!("FileEntryStorage.save_all is not implemented yet"));
    }
}
