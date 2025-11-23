use itertools::Itertools;

use crate::{
    cli::{CheatsheetCli, SearchArgs},
    commands::{command::CommandContext, CommandImplementation, CommandOutputVariant},
    domain::search::EntriesSearchEngine,
};


pub struct SearchCommandImplementation {}


impl CommandImplementation<SearchArgs> for SearchCommandImplementation {
    /// Executes the search command: the engine is loaded based on the command arguments
    fn execute(
        context: &CommandContext,
        _cli: &CheatsheetCli,
        command_config: &SearchArgs,
    ) -> Result<CommandOutputVariant, anyhow::Error> {
        load_search_engine(command_config)
            .and_then(|search_engine| search_impl(context, search_engine.as_ref(), command_config))
    }
}


/// Loads the appropriate search engine based on the command configuration.
///
/// Expected to be extended with more search types in the future.
fn load_search_engine(command_config: &crate::cli::SearchArgs) -> Result<Box<dyn EntriesSearchEngine>, anyhow::Error> {
    match command_config.search_type {
        crate::cli::SearchType::Fuzzy => Ok(Box::new(crate::search::fuzzy::FuzzySearch::new())),
        _ => Err(anyhow::anyhow!(
            "Search type {} is not implemented yet.",
            command_config.search_type.as_ref()
        )),
    }
}


/// The effective implementation of the search command.
///
/// - Builds search parameters from the command configuration
/// - Loads all entries from the storage
/// - Searches the entries using the provided search engine
/// - Returns the matched entries as command output
fn search_impl(
    context: &CommandContext,
    search_engine: &dyn EntriesSearchEngine,
    command_config: &SearchArgs,
) -> Result<CommandOutputVariant, anyhow::Error> {
    let search_parameters = crate::domain::search::SearchParameters {
        query: command_config.query.join(" "),
    };

    let entries = context.storage.load_all()?;

    let search_results = search_engine
        .search(entries.as_ref(), &search_parameters)
        .into_iter()
        .sorted_by_key(|matched_entry| std::cmp::Reverse(matched_entry.score))
        .take(command_config.limit.unwrap_or(usize::MAX))
        .map(|matched_entry| {
            if command_config.full {
                CommandOutputVariant::from(entries[matched_entry.entry_index].clone())
            } else {
                CommandOutputVariant::from(entries[matched_entry.entry_index].command.clone())
            }
        })
        .collect();


    Ok(CommandOutputVariant::Array(search_results))
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rstest::rstest;

    use std::sync::LazyLock;


    use crate::cli::{CheatsheetCli, SearchArgs};
    use crate::commands::command::CommandContext;
    use crate::commands::{CommandImplementation, CommandOutputVariant};
    use crate::domain::entry_storage::MockEntryStorage;
    use crate::domain::search::{MatchedEntry, MockEntriesSearchEngine, SearchParameters};
    use crate::domain::Entry;


    const ENTRIES_IN_DB: LazyLock<Vec<Entry>> = std::sync::LazyLock::new(|| {
        vec![
            crate::domain::entry::Entry {
                title:       "title1".to_string(),
                command:     "command1".to_string(),
                description: Some("description1".to_string()),
            },
            crate::domain::entry::Entry {
                title:       "title2".to_string(),
                command:     "command2".to_string(),
                description: Some("description2".to_string()),
            },
            crate::domain::entry::Entry {
                title:       "title3".to_string(),
                command:     "command3".to_string(),
                description: Some("description3".to_string()),
            },
            crate::domain::entry::Entry {
                title:       "title4".to_string(),
                command:     "command4".to_string(),
                description: Some("description4".to_string()),
            },
        ]
    });


    const MATCHED_ALL_ENTRIES: LazyLock<Vec<MatchedEntry>> = std::sync::LazyLock::new(|| {
        vec![
            MatchedEntry {
                entry_index: 0,
                score:       9,
            },
            MatchedEntry {
                entry_index: 1,
                score:       7,
            },
            MatchedEntry {
                entry_index: 2,
                score:       8,
            },
            MatchedEntry {
                entry_index: 3,
                score:       6,
            },
        ]
    });

    const SEARCH_QUERY: LazyLock<Vec<String>> = LazyLock::new(|| vec!["some".to_string(), "query".to_string()]);
    const EXPECTED_SEARCH_PARAMETERS: LazyLock<SearchParameters> = LazyLock::new(|| SearchParameters {
        query: (*SEARCH_QUERY).join(" "),
    });

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    struct TestCase {
        pub command_config:  SearchArgs,
        pub matched_entries: Vec<MatchedEntry>,
        pub expected_result: CommandOutputVariant,
    }

    #[rstest]
    #[case::full_output_no_limit(TestCase {
        command_config: SearchArgs {
            query:       SEARCH_QUERY.clone(),
            limit:       None,
            search_type: crate::cli::SearchType::Fuzzy,
            full:        true,
        },
        matched_entries: (*MATCHED_ALL_ENTRIES).clone(),
        expected_result: CommandOutputVariant::Array(
            (*MATCHED_ALL_ENTRIES)
                .iter()
                .sorted_by_key(|matched_entry| std::cmp::Reverse(matched_entry.score))
                .map(|e| CommandOutputVariant::from((*ENTRIES_IN_DB)[e.entry_index].clone()))
                .collect::<Vec<CommandOutputVariant>>(),
        ),
    })]
    #[case::full_output_with_limit_2(TestCase {
        command_config: SearchArgs {
            query:       SEARCH_QUERY.clone(),
            limit:       Some(2),
            search_type: crate::cli::SearchType::Fuzzy,
            full:        true,
        },
        matched_entries: (*MATCHED_ALL_ENTRIES).clone(),
        expected_result: CommandOutputVariant::Array(
            (*MATCHED_ALL_ENTRIES)
                .iter()
                .sorted_by_key(|matched_entry| std::cmp::Reverse(matched_entry.score))
                .take(2)
                .map(|e| CommandOutputVariant::from((*ENTRIES_IN_DB)[e.entry_index].clone()))
                .collect::<Vec<CommandOutputVariant>>(),
        ),
    })]
    #[case::not_full_limit_3(TestCase {
        command_config: SearchArgs {
            query:       SEARCH_QUERY.clone(),
            limit:       Some(3),
            search_type: crate::cli::SearchType::Fuzzy,
            full:        false,
        },
        matched_entries: (*MATCHED_ALL_ENTRIES).clone(),
        expected_result: CommandOutputVariant::Array(
            (*MATCHED_ALL_ENTRIES)
                .iter()
                .sorted_by_key(|matched_entry| std::cmp::Reverse(matched_entry.score))
                .take(3)
                .map(|e| CommandOutputVariant::from((*ENTRIES_IN_DB)[e.entry_index].command.clone()))
                .collect::<Vec<CommandOutputVariant>>(),
        ),
    })]
    fn test_search_impl_when_called_then_forms_proper_output(#[case] test_case: TestCase) {
        // Arrange
        let mut storage = MockEntryStorage::new();
        storage.expect_load_all().returning(|| Ok(ENTRIES_IN_DB.clone()));
        let context = CommandContext {
            storage: Box::new(storage),
        };

        // Expectations
        let mut mock_search_engine = MockEntriesSearchEngine::new();
        mock_search_engine
            .expect_search()
            .withf(move |entries, search_parameters| {
                entries == &*ENTRIES_IN_DB && search_parameters == &*EXPECTED_SEARCH_PARAMETERS
            })
            .returning(move |_, _| test_case.matched_entries.clone());

        // Act
        let result = super::search_impl(&context, &mock_search_engine, &test_case.command_config);

        // Assert
        assert_eq!(test_case.expected_result, result.unwrap());
    }

    #[rstest]
    #[case(crate::cli::SearchType::Exact)]
    #[case(crate::cli::SearchType::Substr)]
    fn test_search_command_implementation_when_invalid_engine_then_reports_error(
        #[case] search_type: crate::cli::SearchType,
    ) {
        // Arrange
        let command_config = SearchArgs {
            query:       SEARCH_QUERY.clone(),
            limit:       None,
            search_type: search_type,
            full:        false,
        };

        let mut storage = MockEntryStorage::new();
        storage.expect_load_all().never();
        let context = CommandContext {
            storage: Box::new(storage),
        };

        let cli = CheatsheetCli {
            config:  "some/path".to_string(),
            verbose: 0,
            command: crate::cli::Commands::Search(command_config.clone()),
        };

        // Act
        let result = super::SearchCommandImplementation::execute(&context, &cli, &command_config);

        // Assert
        assert!(result.is_err());
    }
}
