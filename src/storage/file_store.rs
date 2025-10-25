#![allow(dead_code)]

use std::path::{Path, PathBuf};

use crate::domain::{Entry, EntryStorage, EntryStorageError};

/// Minimal file-backed storage stub. Real persistence will be implemented later.
pub struct FileEntryStorage {
    path: PathBuf,
}

impl FileEntryStorage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl EntryStorage for FileEntryStorage {
    type Iter<'a> = std::vec::IntoIter<Entry> where Self: 'a;

    fn load_all(&self) -> Result<Self::Iter<'_>, EntryStorageError> {
        let _ = &self.path;
        Ok(Vec::new().into_iter())
    }

    fn add_entry(&self, entry: Entry) -> Result<(), EntryStorageError> {
        let _ = entry;
        Ok(())
    }

    fn save_all(&self, _entries: &[Entry]) -> Result<(), EntryStorageError> {
        Ok(())
    }
}
