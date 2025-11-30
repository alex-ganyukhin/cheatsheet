use anyhow::Context;

use crate::{
    cli::{AddArgs, CheatsheetCli},
    commands::{command::CommandContext, CommandImplementation, CommandOutputVariant},
    domain::OnConflict,
};


pub struct AddCommandImplementation;


impl CommandImplementation<AddArgs> for AddCommandImplementation {
    fn execute(
        context: &CommandContext,
        _cli: &CheatsheetCli,
        command_config: &AddArgs,
    ) -> Result<CommandOutputVariant, anyhow::Error> {
        context
            .storage
            .add(
                vec![crate::domain::entry::Entry {
                    title:       command_config.title.clone(),
                    command:     command_config.command.clone(),
                    description: command_config.description.clone(),
                }],
                if command_config.replacing {
                    OnConflict::Replace
                } else {
                    OnConflict::Error
                },
            )
            .map(|()| CommandOutputVariant::Value(format!("Entry '{}' added successfully.", command_config.title)))
            .with_context(|| format!("Failed to add entry '{}'", command_config.title))
    }
}


#[cfg(test)]
mod tests {

    use anyhow::anyhow;
    use rstest::rstest;

    use crate::cli::AddArgs;
    use crate::cli::CheatsheetCli;
    use crate::cli::Commands;
    use crate::commands::add::AddCommandImplementation;
    use crate::commands::command::CommandContext;
    use crate::commands::command::CommandImplementation;
    use crate::commands::CommandOutputVariant;
    use crate::domain::entry_storage::MockEntryStorage;
    use crate::domain::OnConflict;


    const SOME_TITLE: &str = "some_title";
    const SOME_COMMAND: &str = "some_command";
    const SOME_DESCRIPTION: &str = "some_description";


    #[test]
    fn test_add_command_implementation_when_add_failed_then_passes_error_as_is() {
        // Arrange
        let expected_errors_chain = vec!["Failed to add entry 'some_title'", "UNDERLYING ERROR"];

        let add_args = AddArgs {
            title:       SOME_TITLE.to_string(),
            command:     SOME_COMMAND.to_string(),
            description: Some(SOME_DESCRIPTION.to_string()),
            replacing:   false,
        };

        let cli = CheatsheetCli {
            config:  "".to_string(),
            verbose: 0,
            command: Commands::Add(add_args.clone()),
        };

        let mut storage = MockEntryStorage::new();
        storage
            .expect_add()
            .withf(move |entries, on_conflict| {
                entries.len() == 1
                    && entries[0].title == SOME_TITLE.to_string()
                    && entries[0].command == SOME_COMMAND.to_string()
                    && entries[0].description == Some(SOME_DESCRIPTION.to_string())
                    && *on_conflict == crate::domain::OnConflict::Error
            })
            .returning(|_, _| Err(anyhow!("UNDERLYING ERROR")));
        let context = CommandContext {
            storage: Box::new(storage),
        };

        // Act
        let result = AddCommandImplementation::execute(&context, &cli, &add_args);

        // Assert
        let chain = result.unwrap_err().chain().map(|c| c.to_string()).collect::<Vec<_>>();
        assert_eq!(expected_errors_chain, chain);
    }


    #[test]
    fn test_add_command_implementation_when_add_succeed_then_prints_success_message() {
        // Arrange
        let expected_success_message =
            CommandOutputVariant::Value(format!("Entry '{}' added successfully.", SOME_TITLE));

        let add_args = AddArgs {
            title:       SOME_TITLE.to_string(),
            command:     SOME_COMMAND.to_string(),
            description: Some(SOME_DESCRIPTION.to_string()),
            replacing:   false,
        };

        let cli = CheatsheetCli {
            config:  "".to_string(),
            verbose: 0,
            command: Commands::Add(add_args.clone()),
        };

        let mut storage = MockEntryStorage::new();
        storage
            .expect_add()
            .withf(move |entries, on_conflict| {
                entries.len() == 1
                    && entries[0].title == SOME_TITLE.to_string()
                    && entries[0].command == SOME_COMMAND.to_string()
                    && entries[0].description == Some(SOME_DESCRIPTION.to_string())
                    && *on_conflict == crate::domain::OnConflict::Error
            })
            .returning(|_, _| Ok(()));
        let context = CommandContext {
            storage: Box::new(storage),
        };

        // Act
        let result = AddCommandImplementation::execute(&context, &cli, &add_args);

        // Assert
        assert_eq!(expected_success_message, result.unwrap());
    }


    #[rstest]
    #[case::if_replacing_then_replace_strategy(true, OnConflict::Replace)]
    #[case::if_not_replacing_then_error_strategy(false, OnConflict::Error)]
    fn test_add_command_implementation_given_replacing_value_expect_proper_storage_request(
        #[case] replacing_value: bool,
        #[case] expected_on_conflict: OnConflict,
    ) {
        // Arrange
        let add_args = AddArgs {
            title:       SOME_TITLE.to_string(),
            command:     SOME_COMMAND.to_string(),
            description: Some(SOME_DESCRIPTION.to_string()),
            replacing:   replacing_value,
        };

        let cli = CheatsheetCli {
            config:  "".to_string(),
            verbose: 0,
            command: Commands::Add(add_args.clone()),
        };

        let mut storage = MockEntryStorage::new();
        storage
            .expect_add()
            .withf(move |_, on_conflict| *on_conflict == expected_on_conflict)
            .returning(|_, _| Ok(()));
        let context = CommandContext {
            storage: Box::new(storage),
        };

        // Act
        let result = AddCommandImplementation::execute(&context, &cli, &add_args);

        // Assert
        assert!(result.is_ok());
    }
}
