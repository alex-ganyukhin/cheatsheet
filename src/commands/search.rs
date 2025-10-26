#![allow(dead_code)]

use crate::domain::search::EntriesSearch;
use crate::domain::{Entry, EntryStorage, EntryStorageError};

pub struct SearchCommand;

impl SearchCommand {
    pub fn run<S: EntryStorage>(
        storage: &S,
        matcher: &dyn EntriesSearch,
        query: &str,
    ) -> Result<Vec<Entry>, EntryStorageError> {
        let entries: Vec<Entry> = storage.load_all()?.collect();
        Ok(matcher.search(&entries, query))
    }
}
