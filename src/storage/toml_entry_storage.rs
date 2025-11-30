use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};


use anyhow::Context;
use strum_macros::{AsRefStr, Display};

use crate::storage::serde_models::TomlEntryStorageDef;

use crate::domain::{Entry, EntryStorage, OnConflict};


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

    /// Used when the file cannot be written to due to any error
    #[strum(to_string = "CannotWrite {0}")]
    CannotWrite(String),
}


impl std::error::Error for TomlEntryStorageError {}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


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

    /// Stores all entries into the TOML file, replacing any existing content.
    fn store_replacing(&self, entries: Vec<Entry>) -> Result<(), anyhow::Error> {
        let storage_def = TomlEntryStorageDef {
            entry: entries
                .into_iter()
                .map(move |e| crate::storage::serde_models::TomlEntry {
                    title:       e.title,
                    command:     e.command,
                    description: e.description,
                })
                .collect(),
        };

        let toml_string = toml::to_string(&storage_def).with_context(|| {
            TomlEntryStorageError::InvalidFormat("Failed to serialize entries to TOML format.".to_string())
        })?;

        fs::write(self.path(), toml_string).with_context(|| {
            TomlEntryStorageError::CannotWrite(format!("Cannot write to file at path {}", self.path.display()))
        })
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


    fn add(&self, entries: Vec<Entry>, on_conflict: OnConflict) -> Result<(), anyhow::Error> {
        let mut existing_entries = self.load_all()?;
        let mut entries_to_add = entries
            .into_iter()
            .map(move |e| (e.title.clone(), e))
            .collect::<HashMap<_, _>>();

        // 1. Handle conflicts
        for existing_entry in existing_entries.iter_mut() {
            if let Some(new_entry) = entries_to_add.remove(&existing_entry.title) {
                match on_conflict {
                    OnConflict::Error => {
                        return Err(anyhow::anyhow!(
                            "Entry with title '{}' already exists.",
                            existing_entry.title
                        ));
                    }
                    OnConflict::Replace => {
                        *existing_entry = new_entry;
                    }
                }
            }
        }

        // 2. Append new entries
        let mut final_entries = existing_entries;
        final_entries.extend(entries_to_add.into_values());

        self.store_replacing(final_entries)
    }


    fn remove(&self, titles: Vec<String>) -> Result<usize, anyhow::Error> {
        let mut entries_in_storage = self.load_all()?;
        let titles = titles.into_iter().collect::<HashSet<_>>();

        let initial_count = entries_in_storage.len();
        entries_in_storage.retain(|e| !titles.contains(&e.title));
        let amount_removed = initial_count - entries_in_storage.len();

        self.store_replacing(entries_in_storage)?;

        Ok(amount_removed)
    }
}
