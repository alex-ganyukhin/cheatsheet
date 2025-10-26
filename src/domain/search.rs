use crate::domain::entry::Entry;

pub trait EntriesSearch {
    /// Searches through the provided entries using the given query string.
    /// Returns a vector of entries that match the query.
    ///
    /// # Example
    /// ```
    /// let result = some_search.search(entries, "query");
    /// ```
    fn search(&self, entries: &[Entry], _query: &str) -> Vec<Entry>;
}
