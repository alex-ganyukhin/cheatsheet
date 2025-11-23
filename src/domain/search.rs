use crate::domain::entry::Entry;


#[derive(PartialEq, Clone, Debug)]
pub struct MatchedEntry {
    /// The index of the entry in the original entries list.
    pub entry_index: usize,

    /// The score indicating how well the entry matches the search query.
    /// It is not defined what the score range is, how it is calculated, etc.
    /// The only guarantee is that higher score means better match.
    pub score: i64,
}


#[derive(PartialEq, Clone, Debug)]
pub struct SearchParameters {
    /// The query string to search for.
    pub query: String,
}


impl SearchParameters {
    pub fn matching_query(query: String) -> Self {
        SearchParameters { query }
    }
}


#[mockall::automock]
pub trait EntriesSearchEngine {
    /// Searches through the provided entries using the given query string.
    ///
    /// ### Return
    /// - A vector of `MatchedEntry` structs.
    /// - The order of entries in the returned vector is not guaranteed.
    ///
    /// ### Example
    /// ```
    /// use cheatsheet::domain::search::SearchParameters;
    /// let search_parameters = SearchParameters::matching_query("query".into());
    /// ```
    fn search(&self, entries: &[Entry], search_parameters: &SearchParameters) -> Vec<MatchedEntry>;
}
