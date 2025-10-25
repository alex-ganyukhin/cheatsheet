#![allow(dead_code)]

use crate::domain::{Entry, EntryStorage, EntryStorageError};

pub struct AddCommand;

impl AddCommand {
    pub fn run<S: EntryStorage>(storage: &S, entry: Entry) -> Result<(), EntryStorageError> {
        storage.add_entry(entry)
    }
}
