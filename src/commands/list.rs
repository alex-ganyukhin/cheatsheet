#![allow(dead_code)]

use crate::domain::{Entry, EntryStorage, EntryStorageError};

pub struct ListCommand;

impl ListCommand {
    pub fn run<S: EntryStorage>(storage: &S) -> Result<Vec<Entry>, EntryStorageError> {
        storage.load_all().map(|iter| iter.collect::<Vec<_>>())
    }
}
