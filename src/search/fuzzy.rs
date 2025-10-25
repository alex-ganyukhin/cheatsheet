#![allow(dead_code)]

use crate::domain::Entry;

/// Placeholder fuzzy searcher that returns the provided entries unchanged.
pub struct FuzzySearch;

impl FuzzySearch {
    pub fn new() -> Self {
        Self
    }

    pub fn search(&self, entries: &[Entry], _query: &str) -> Vec<Entry> {
        entries.to_vec()
    }
}
