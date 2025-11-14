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
        let search_engine = load_search_engine(command_config)?;
        search_impl(context, search_engine.as_ref(), command_config)
    }
}


/// Loads the appropriate search engine based on the command configuration.
///
/// Expected to be extended with more search types in the future.
fn load_search_engine(command_config: &crate::cli::SearchArgs) -> Result<Box<dyn EntriesSearchEngine>, anyhow::Error> {
    match command_config.search_type {
        crate::cli::SearchType::Fuzzy => Ok(Box::new(crate::search::fuzzy::FuzzySearch::new())),
        _ => Err(anyhow::anyhow!("Exact search is not implemented yet.")),
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
        query:       command_config.query.join(" "),
        max_results: command_config.limit,
    };

    let entries = context.storage.load_all()?;

    let search_results = search_engine
        .search(entries.as_ref(), &search_parameters)
        .into_iter()
        .map(|e| e.entry.into())
        .collect();


    Ok(CommandOutputVariant::Array(search_results))
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use crate::cli::SearchArgs;
    use crate::commands::command::CommandContext;
    use crate::commands::CommandOutputVariant;
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
        ]
    });

    const MATCHED_ENTRIES: LazyLock<Vec<MatchedEntry>> = std::sync::LazyLock::new(|| {
        vec![
            MatchedEntry {
                entry: ENTRIES_IN_DB[0].clone(),
                score: 0.9,
            },
            MatchedEntry {
                entry: ENTRIES_IN_DB[2].clone(),
                score: 0.8,
            },
        ]
    });

    const EXPECTED_RESULT: LazyLock<CommandOutputVariant> = LazyLock::new(|| {
        let items = MATCHED_ENTRIES
            .iter()
            .map(|e| CommandOutputVariant::from(e.entry.clone()))
            .collect::<Vec<CommandOutputVariant>>();

        CommandOutputVariant::Array(items)
    });


    #[test]
    fn test_search_impl_given_search_engine_nad_config_then_forms_proper_request_and_returns_proper_results() {
        // Arrange
        let search_query = "some query".to_string();
        let search_limit = Some(10usize);

        let command_config = SearchArgs {
            query:       vec!["some".to_string(), "query".to_string()],
            limit:       search_limit,
            search_type: crate::cli::SearchType::Fuzzy,
        };

        let expected_search_parameters = SearchParameters {
            query:       search_query.clone(),
            max_results: search_limit,
        };

        let mut mock_search_engine = MockEntriesSearchEngine::new();
        mock_search_engine
            .expect_search()
            .withf(move |entries, search_parameters| {
                entries == &*ENTRIES_IN_DB && search_parameters == &expected_search_parameters
            })
            .returning(move |_, _| MATCHED_ENTRIES.clone());

        let mut storage = MockEntryStorage::new();
        storage.expect_load_all().returning(|| Ok(ENTRIES_IN_DB.clone()));

        let context = CommandContext {
            storage: Box::new(storage),
        };

        // Act
        let result = super::search_impl(&context, &mock_search_engine, &command_config);

        // Assert
        assert_eq!(*EXPECTED_RESULT, result.unwrap());
    }
}
