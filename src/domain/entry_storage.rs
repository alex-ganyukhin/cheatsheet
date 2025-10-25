use crate::domain::entry::Entry;

/// Abstraction over persistence for cheat sheet entries.
///
/// Keeping this trait in the domain layer lets the command handlers depend on a
/// storage contract without assuming any concrete technology (TOML file,
/// database, etc.).
pub trait EntryStorage {
    type Iter<'a>: Iterator<Item = Entry> + 'a
    where
        Self: 'a;

    fn load_all(&self) -> Result<Self::Iter<'_>, EntryStorageError>;
    fn add_entry(&self, entry: Entry) -> Result<(), EntryStorageError>;
    fn save_all(&self, entries: &[Entry]) -> Result<(), EntryStorageError>;
}

/// Domain-friendly error for persistence failures.
#[derive(Debug)]
pub enum EntryStorageError {
    ReadFailed(String),
    WriteFailed(String),
}

impl EntryStorageError {
    pub fn read_failed<T: Into<String>>(message: T) -> Self {
        Self::ReadFailed(message.into())
    }

    pub fn write_failed<T: Into<String>>(message: T) -> Self {
        Self::WriteFailed(message.into())
    }
}

impl std::fmt::Display for EntryStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryStorageError::ReadFailed(msg) => write!(f, "failed to load entries: {}", msg),
            EntryStorageError::WriteFailed(msg) => write!(f, "failed to persist entries: {}", msg),
        }
    }
}

impl std::error::Error for EntryStorageError {}
