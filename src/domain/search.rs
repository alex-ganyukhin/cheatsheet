use crate::domain::entry::Entry;


#[derive(PartialEq, Clone, Debug)]
pub struct MatchedEntry {
    /// The matched entry
    pub entry: Entry,

    /// The score indicating how well the entry matches the search query.
    /// It is not defined what the score range is, how it is calculated, etc.
    /// The only guarantee is that higher score means better match.
    pub score: f32,
}


#[derive(PartialEq, Clone, Debug)]
pub struct SearchParameters {
    /// The query string to search for.
    pub query:       String,
    pub max_results: Option<usize>,
}


impl SearchParameters {
    pub fn return_all(query: String) -> Self {
        SearchParameters {
            query,
            max_results: None,
        }
    }
}


#[mockall::automock]
pub trait EntriesSearchEngine {
    /// Searches through the provided entries using the given query string.
    /// Returns a vector of entries that match the query.
    ///
    /// # Example
    /// ```
    /// let result = some_search.search(entries, &SearchParameters::return_all("query".into()));
    /// ```


    fn search(&self, entries: &[Entry], search_parameters: &SearchParameters) -> Vec<MatchedEntry>;
}
