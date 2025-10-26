#![allow(dead_code)]

use crate::domain::search::EntriesSearch;
use crate::domain::Entry;

/// Placeholder fuzzy searcher that returns the provided entries unchanged.
pub struct FuzzySearch;

impl EntriesSearch for FuzzySearch {
    fn search(&self, entries: &[Entry], _query: &str) -> Vec<Entry> {
        entries.to_vec()
    }
}

impl FuzzySearch {
    pub fn new() -> Self {
        FuzzySearch
    }
}
