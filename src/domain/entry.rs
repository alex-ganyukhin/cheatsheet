/// A single entry in the cheat sheet
#[derive(Clone, Debug, PartialEq, Eq)]

pub struct Entry {
    pub title:       String,
    pub command:     String,
    pub description: Option<String>,
}

impl Entry {
    /// Convenience constructor that normalizes the textual fields into owned strings.
    pub fn new<T, U, V>(title: T, command: U, description: Option<V>) -> Self
    where
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        Self {
            title:       title.into(),
            command:     command.into(),
            description: description.map(Into::into),
        }
    }
}
