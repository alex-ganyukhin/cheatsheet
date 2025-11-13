#![allow(dead_code)]

use crate::domain::search::{EntriesSearch, MatchedEntry, SearchParameters};
use crate::domain::Entry;

/// Placeholder fuzzy searcher that returns the provided entries unchanged.
pub struct FuzzySearch;

impl EntriesSearch for FuzzySearch {
    fn search(&self, _entries: &[Entry], _search_parameters: &SearchParameters) -> Vec<MatchedEntry> {
        Vec::new()
    }
}

impl FuzzySearch {
    pub fn new() -> Self {
        FuzzySearch
    }
}
