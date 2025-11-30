use crate::domain::entry::Entry;

/// The strategy to use when adding entries to the storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnConflict {
    /// In case of conflict, existing entries should be replaced.
    Replace,

    /// In case of conflict, existing entries should be kept, and new ones ignored.
    Error,
}

/// Abstraction over persistence for cheat sheet entries.
///
/// Keeping this trait in the domain layer lets the command handlers depend on a
/// storage contract without assuming any concrete technology (TOML file,
/// database, etc.).
#[mockall::automock]
pub trait EntryStorage {
    /// Loads all entries from the storage.
    ///
    fn load_all(&self) -> Result<Vec<Entry>, anyhow::Error>;

    /// Adds new entries to the storage.
    ///
    /// ### Parameters
    /// - `entries`: The entries to add.
    /// - `on_conflict`: Strategy to use in case of conflict with existing entries.
    ///
    /// ### Returns
    /// An error if the operation failed, or details about added/replaced entries.
    fn add(&self, entries: Vec<Entry>, on_conflict: OnConflict) -> Result<(), anyhow::Error>;

    /// Removes entries from the storage.
    ///
    /// ### Returns
    /// The number of removed entries.
    fn remove(&self, titles: Vec<String>) -> Result<usize, anyhow::Error>;
}
