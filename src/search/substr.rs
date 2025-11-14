use crate::domain::search::{EntriesSearchEngine, MatchedEntry, SearchParameters};
use crate::domain::Entry;


pub struct SubstrSearch;


impl EntriesSearchEngine for SubstrSearch {
    fn search(&self, _entries: &[Entry], _search_parameters: &SearchParameters) -> Vec<MatchedEntry> {
        Vec::new()
    }
}


impl SubstrSearch {
    pub fn new() -> Self {
        SubstrSearch
    }
}
