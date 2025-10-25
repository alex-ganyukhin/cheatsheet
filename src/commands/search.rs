#![allow(dead_code)]

use crate::domain::{Entry, EntryStorage, EntryStorageError};
use crate::search::FuzzySearch;

pub struct SearchCommand;

impl SearchCommand {
    pub fn run<S: EntryStorage>(
        storage: &S,
        matcher: &FuzzySearch,
        query: &str,
    ) -> Result<Vec<Entry>, EntryStorageError> {
        let entries: Vec<Entry> = storage.load_all()?.collect();
        Ok(matcher.search(&entries, query))
    }
}
