use crate::domain::entry::Entry;

/// Abstraction over persistence for cheat sheet entries.
///
/// Keeping this trait in the domain layer lets the command handlers depend on a
/// storage contract without assuming any concrete technology (TOML file,
/// database, etc.).
#[mockall::automock]
pub trait EntryStorage {
    fn load_all(&self) -> Result<Vec<Entry>, anyhow::Error>;
    fn add_entry(&self, entry: Entry) -> Result<(), anyhow::Error>;
    fn save_all(&self, entries: &[Entry]) -> Result<(), anyhow::Error>;
}
