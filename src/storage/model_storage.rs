use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};


use anyhow::Context;
use strum_macros::{AsRefStr, Display};

use crate::storage::serde_models::{EntryModel, EntryModelStorageDef};

use crate::domain::{Entry, EntryStorage, OnConflict};


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


#[derive(Debug, AsRefStr, Display)]


pub enum ModelStorageIOError {
    /// Used when no more specific error variant is applicable.
    #[strum(to_string = "GenericError {0}")]
    Generic(String),

    /// Used to indicate that operation is failed due to unknown/unrecognized format
    #[strum(to_string = "InvalidFormat {0}")]
    InvalidFormat(String),

    /// Used when the file cannot be opened due to any error
    #[strum(to_string = "CannotOpen {0}")]
    CannotOpen(String),

    /// Used when the file cannot be written to due to any error
    #[strum(to_string = "CannotWrite {0}")]
    CannotWrite(String),

    #[strum(to_string = "Duplicate {0}")]
    Duplicate(String),
}


impl std::error::Error for ModelStorageIOError {}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// Very simple raw storage abstraction for storing/loading serde models.
/// Expected to be used, when storing model in one-shot makes sense (e.g. small files).
#[mockall::automock]
pub trait ModelStorageIO {
    fn store(&self, entries: EntryModelStorageDef) -> Result<(), anyhow::Error>;
    fn load(&self) -> Result<EntryModelStorageDef, anyhow::Error>;
}


/// Implements `EntryStorage` over any `ModelStorageIO` implementation.
pub struct ModelStorageIOEntryStorageAdapter<TModelStorageIO>
where
    TModelStorageIO: ModelStorageIO,
{
    model_io: TModelStorageIO,
}

impl<TModelStorageIO> ModelStorageIOEntryStorageAdapter<TModelStorageIO>
where
    TModelStorageIO: ModelStorageIO,
{
    pub fn new(model_io: TModelStorageIO) -> Self {
        Self { model_io }
    }
}


impl<TModelStorageIO> EntryStorage for ModelStorageIOEntryStorageAdapter<TModelStorageIO>
where
    TModelStorageIO: ModelStorageIO,
{
    fn load_all(&self) -> Result<Vec<Entry>, anyhow::Error> {
        let loaded_model = self.model_io.load()?;

        loaded_model
            .entry
            .into_iter()
            .map(|e| {
                Ok(Entry {
                    title:       e.title,
                    command:     e.command,
                    description: e.description,
                })
            })
            .collect()
    }


    fn add(&self, entries: Vec<Entry>, on_conflict: OnConflict) -> Result<(), anyhow::Error> {
        let mut loaded_model = self.model_io.load()?;

        let entries_to_add_amount = entries.len();
        let mut entries_to_add = entries
            .into_iter()
            .map(move |e| (e.title.clone(), e))
            .collect::<HashMap<_, _>>();

        if entries_to_add.len() != entries_to_add_amount {
            return Err(ModelStorageIOError::Duplicate("Duplicate titles found in entries to add.".to_string()).into());
        }

        // 1. Handle conflicts
        for existing_entry in loaded_model.entry.iter_mut() {
            if let Some(new_entry) = entries_to_add.remove(&existing_entry.title) {
                match on_conflict {
                    OnConflict::Error => {
                        return Err(ModelStorageIOError::Duplicate(format!(
                            "Entry with title '{}' already exists.",
                            existing_entry.title
                        ))
                        .into());
                    }
                    OnConflict::Replace => {
                        *existing_entry = EntryModel {
                            title:       new_entry.title,
                            command:     new_entry.command,
                            description: new_entry.description,
                        };
                    }
                }
            }
        }

        // 2. Append new entries
        loaded_model
            .entry
            .extend(entries_to_add.into_values().map(|e| EntryModel {
                title:       e.title,
                command:     e.command,
                description: e.description,
            }));

        self.model_io.store(loaded_model)
    }


    fn remove(&self, titles: Vec<String>) -> Result<usize, anyhow::Error> {
        let mut loaded_model = self.model_io.load()?;


        let titles_set = titles.into_iter().collect::<HashSet<_>>();

        let initial_count = loaded_model.entry.len();
        loaded_model.entry.retain(|e| !titles_set.contains(&e.title));
        let amount_removed = initial_count - loaded_model.entry.len();


        self.model_io.store(loaded_model)?;
        Ok(amount_removed)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct TomlModelStorageIO {
    path: PathBuf,
}

impl TomlModelStorageIO {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl ModelStorageIO for TomlModelStorageIO {
    fn store(&self, entries: EntryModelStorageDef) -> Result<(), anyhow::Error> {
        let toml_string = toml::to_string(&entries).with_context(|| {
            ModelStorageIOError::InvalidFormat("Failed to serialize entries to TOML format.".to_string())
        })?;

        fs::write(self.path(), toml_string).with_context(|| {
            ModelStorageIOError::CannotWrite(format!("Cannot write to file at path {}", self.path.display()))
        })
    }

    fn load(&self) -> Result<EntryModelStorageDef, anyhow::Error> {
        let loaded_toml_string: String = fs::read_to_string(&self.path).with_context(|| {
            ModelStorageIOError::CannotOpen(format!("Cannot open file at path {}", self.path.display()))
        })?;

        toml::from_str(&loaded_toml_string).with_context(|| {
            ModelStorageIOError::InvalidFormat(format!(
                "Cannot parse TOML content from file at path {}",
                self.path.display()
            ))
        })
    }
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::domain::Entry;
    use crate::domain::EntryStorage;
    use crate::storage::model_storage::MockModelStorageIO;
    use crate::storage::serde_models::{EntryModel, EntryModelStorageDef};
    use crate::storage::ModelStorageIOEntryStorageAdapter;
    use crate::storage::ModelStorageIOError;

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    #[rstest]
    #[case(
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title:       "title1".to_string(),
                    command:     "command1".to_string(),
                    description: Some("description1".to_string()),
                },
                EntryModel {
                    title:       "title2".to_string(),
                    command:     "command2".to_string(),
                    description: None,
                },
            ],
        },
        vec![
            Entry {
                title:       "title1".to_string(),
                command:     "command1".to_string(),
                description: Some("description1".to_string()),
            },
            Entry {
                title:       "title2".to_string(),
                command:     "command2".to_string(),
                description: None,
            },
        ],
    )]
    #[case(
        EntryModelStorageDef { entry: vec![] },
        vec![],
    )]
    fn test_load_all_given_storage_content_expect_proper_entries_loaded(
        #[case] model_initially: EntryModelStorageDef,
        #[case] expected_entries: Vec<Entry>,
    ) {
        // Arrange
        let mut model_storage_io_mock = MockModelStorageIO::new();
        model_storage_io_mock
            .expect_load()
            .returning(move || Ok(model_initially.clone()));

        let entry_storage = ModelStorageIOEntryStorageAdapter::new(model_storage_io_mock);

        // Act
        let loaded_entries = entry_storage.load_all().unwrap();

        // Assert
        assert_eq!(loaded_entries, expected_entries);
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    #[rstest]
    #[case(
        EntryModelStorageDef { entry: vec![] },
        vec![
            Entry {
                title: "title1".to_string(),
                command: "command1".to_string(),
                description: None,
            }
        ],
        crate::domain::OnConflict::Error,
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                }
            ]
        }
    )]
    #[case(
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                }
            ]
        },
        vec![
            Entry {
                title: "title2".to_string(),
                command: "command2".to_string(),
                description: None,
            }
        ],
        crate::domain::OnConflict::Error,
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                },
                EntryModel {
                    title: "title2".to_string(),
                    command: "command2".to_string(),
                    description: None,
                }
            ]
        }
    )]
    #[case::replace_existing_entry(
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: Some("old".to_string()),
                },
                EntryModel {
                    title: "title2".to_string(),
                    command: "command2".to_string(),
                    description: None,
                }
            ]
        },
        vec![
            Entry {
                title: "title1".to_string(),
                command: "command1_new".to_string(),
                description: Some("new".to_string()),
            }
        ],
        crate::domain::OnConflict::Replace,
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1_new".to_string(),
                    description: Some("new".to_string()),
                },
                EntryModel {
                    title: "title2".to_string(),
                    command: "command2".to_string(),
                    description: None,
                }
            ]
        }
    )]
    fn test_add_given_initial_contents_and_parameters_expect_proper_final_contents(
        #[case] model_initially: EntryModelStorageDef,
        #[case] entries_to_add: Vec<Entry>,
        #[case] on_conflict: crate::domain::OnConflict,
        #[case] expected_final_model: EntryModelStorageDef,
    ) {
        // Arrange
        let mut model_storage_io_mock = MockModelStorageIO::new();

        model_storage_io_mock
            .expect_load()
            .returning(move || Ok(model_initially.clone()));

        model_storage_io_mock
            .expect_store()
            .withf(move |stored_model| stored_model == &expected_final_model)
            .returning(|_| Ok(()));

        let entry_storage = ModelStorageIOEntryStorageAdapter::new(model_storage_io_mock);

        // Act
        let result = entry_storage.add(entries_to_add, on_conflict);

        // Assert
        assert!(result.is_ok());
    }


    #[rstest]
    #[case::cannot_open_file(
        (|| {
            let mut model_storage_io_mock = MockModelStorageIO::new();

            model_storage_io_mock
            .expect_load()
            .returning(move || Err(ModelStorageIOError::CannotOpen("File not found".to_string()).into()));

            return model_storage_io_mock;
        })(),
        vec![
            Entry {
                title: "title1".to_string(),
                command: "command1".to_string(),
                description: None,
            }
        ],
        crate::domain::OnConflict::Error,
        ModelStorageIOError::CannotOpen("File not found".to_string()).into(),
    )]
    #[case::adding_entries_contains_duplicates(
        (|| {
            let mut model_storage_io_mock = MockModelStorageIO::new();

            model_storage_io_mock
            .expect_load()
            .returning(move || Ok(EntryModelStorageDef { entry: vec![] }));
            return model_storage_io_mock;
        })(),
        vec![
            Entry {
                title: "title1".to_string(),
                command: "command1".to_string(),
                description: None,
            },
            Entry {
                title: "title1".to_string(),
                command: "command1_dup".to_string(),
                description: None,
            }
        ],
        crate::domain::OnConflict::Error,
        ModelStorageIOError::Duplicate("Duplicate titles found in entries to add.".to_string()).into(),
    )]
    #[case::conflict_on_existing_entry(
        (|| {
            let mut model_storage_io_mock = MockModelStorageIO::new();

            model_storage_io_mock
            .expect_load()
            .returning(move || Ok(EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                }
            ]
            }));
            return model_storage_io_mock;
        })(),
        vec![
            Entry {
                title: "title1".to_string(),
                command: "command1_new".to_string(),
                description: None,
            }
        ],
        crate::domain::OnConflict::Error,
        ModelStorageIOError::Duplicate("Entry with title 'title1' already exists.".to_string()).into(),
    )]
    #[case::cannot_write_file(
        (|| {
            let mut model_storage_io_mock = MockModelStorageIO::new();

            model_storage_io_mock
                .expect_load()
                .returning(move || Ok(EntryModelStorageDef { entry: vec![] }));

            model_storage_io_mock
                .expect_store()
                .returning(move |_| {
                    Err(ModelStorageIOError::CannotWrite("Cannot write to file at path /some/path".to_string()).into())
                });

            return model_storage_io_mock;
        })(),
        vec![
            Entry {
                title: "title1".to_string(),
                command: "command1".to_string(),
                description: None,
            }
        ],
        crate::domain::OnConflict::Error,
        ModelStorageIOError::CannotWrite("Cannot write to file at path /some/path".to_string()).into(),
    )]
    fn test_add_given_error_condition_expect_error(
        #[case] mocked_storage_with_expectations: MockModelStorageIO,
        #[case] entries_to_add: Vec<Entry>,
        #[case] on_conflict: crate::domain::OnConflict,
        #[case] expected_error: anyhow::Error,
    ) {
        // Arrange
        let entry_storage = ModelStorageIOEntryStorageAdapter::new(mocked_storage_with_expectations);

        // Act
        let result = entry_storage.add(entries_to_add, on_conflict);

        // Assert
        assert_eq!(format!("{}", result.unwrap_err()), format!("{}", expected_error));
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    #[rstest]
    #[case::regular(
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                },
                EntryModel {
                    title: "title2".to_string(),
                    command: "command2".to_string(),
                    description: None,
                },
                EntryModel {
                    title: "title3".to_string(),
                    command: "command3".to_string(),
                    description: None,
                },
            ],
        },
        vec!["title2".to_string(), "title3".to_string()],
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                },
            ],
        },
        2,
    )]
    #[case::no_titles_to_remove(
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                },
            ],
        },
        vec![],
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                },
            ],
        },
        0,
    )]
    #[case::titles_not_existing(
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                },
            ],
        },
        vec!["title2".to_string(), "title3".to_string()],
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                },
            ],
        },
        0,
    )]
    #[case::duplicate_titles_to_remove(
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                },
                EntryModel {
                    title: "title2".to_string(),
                    command: "command2".to_string(),
                    description: None,
                },
            ],
        },
        vec!["title2".to_string(), "title2".to_string(), "title2".to_string()],
        EntryModelStorageDef {
            entry: vec![
                EntryModel {
                    title: "title1".to_string(),
                    command: "command1".to_string(),
                    description: None,
                },
            ],
        },
        1,
    )]
    fn test_remove_given_titles_expect_proper_amount_removed(
        #[case] model_initially: EntryModelStorageDef,
        #[case] titles_to_remove: Vec<String>,
        #[case] expected_final_model: EntryModelStorageDef,
        #[case] expected_amount_removed: usize,
    ) {
        // Arrange
        let mut model_storage_io_mock = MockModelStorageIO::new();

        model_storage_io_mock
            .expect_load()
            .returning(move || Ok(model_initially.clone()));

        model_storage_io_mock
            .expect_store()
            .withf(move |stored_model| stored_model == &expected_final_model)
            .returning(|_| Ok(()));

        let entry_storage = ModelStorageIOEntryStorageAdapter::new(model_storage_io_mock);

        // Act
        let amount_removed = entry_storage.remove(titles_to_remove).unwrap();

        // Assert
        assert_eq!(amount_removed, expected_amount_removed);
    }
}
