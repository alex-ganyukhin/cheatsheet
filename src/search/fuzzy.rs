#![allow(dead_code)]

use crate::domain::search::{EntriesSearch, MatchedEntry};
use crate::domain::Entry;

/// Placeholder fuzzy searcher that returns the provided entries unchanged.
pub struct FuzzySearch;

impl EntriesSearch for FuzzySearch {
    fn search(&self, _entries: &[Entry], _query: &str) -> Vec<MatchedEntry> {
        Vec::new()
    }
}

impl FuzzySearch {
    pub fn new() -> Self {
        FuzzySearch
    }
}
