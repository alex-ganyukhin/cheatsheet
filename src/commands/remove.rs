#![allow(dead_code)]

use crate::domain::{EntryStorage, EntryStorageError};

pub struct RemoveCommand;

impl RemoveCommand {
    pub fn run<S: EntryStorage>(storage: &S, title: &str) -> Result<(), EntryStorageError> {
        let entries = storage.load_all()?;
        let filtered: Vec<_> = entries
            .into_iter()
            .filter(|entry| entry.title != title)
            .collect();
        storage.save_all(&filtered)
    }
}
