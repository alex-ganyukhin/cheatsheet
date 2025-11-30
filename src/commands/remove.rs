use anyhow::Context;

use crate::{
    cli::{CheatsheetCli, RemoveArgs},
    commands::{command::CommandContext, CommandImplementation, CommandOutputVariant},
};


pub struct RemoveCommandImplementation;


impl CommandImplementation<RemoveArgs> for RemoveCommandImplementation {
    fn execute(
        context: &CommandContext,
        _cli: &CheatsheetCli,
        command_config: &RemoveArgs,
    ) -> Result<CommandOutputVariant, anyhow::Error> {
        context
            .storage
            .remove(command_config.title.clone())
            .map(|amount_removed| {
                CommandOutputVariant::Value(format!(
                    "Effectively removed {} entries (requested {}).",
                    amount_removed,
                    command_config.title.len()
                ))
            })
            .with_context(|| format!("Failed to remove {} entries.", command_config.title.len()))
    }
}


#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::commands::command::CommandImplementation;
    use crate::commands::CommandOutputVariant;
    use crate::{
        cli::{CheatsheetCli, Commands, RemoveArgs},
        commands::{command::CommandContext, RemoveCommandImplementation},
        domain::entry_storage::MockEntryStorage,
    };
    use anyhow::anyhow;


    const SOME_TITLES: LazyLock<Vec<String>> =
        LazyLock::new(|| vec!["some_title".to_string(), "another_title".to_string()]);


    #[test]
    fn test_remove_command_implementation_when_remove_failed_then_passes_error_as_is() {
        // Arrange
        let expected_errors_chain = vec![
            format!("Failed to remove {} entries.", SOME_TITLES.len()),
            "UNDERLYING ERROR".to_string(),
        ];

        let remove_args = RemoveArgs {
            title: SOME_TITLES.clone(),
        };

        let cli = CheatsheetCli {
            config:  "".to_string(),
            verbose: 0,
            command: Commands::Remove(remove_args.clone()),
        };

        let mut storage = MockEntryStorage::new();
        storage
            .expect_remove()
            .withf(move |entries| entries == &*SOME_TITLES)
            .returning(|_| Err(anyhow!("UNDERLYING ERROR")));
        let context = CommandContext {
            storage: Box::new(storage),
        };

        // Act
        let result = RemoveCommandImplementation::execute(&context, &cli, &remove_args);

        // Assert
        let chain = result.unwrap_err().chain().map(|c| c.to_string()).collect::<Vec<_>>();
        assert_eq!(expected_errors_chain, chain);
    }


    #[test]
    fn test_add_command_implementation_when_remove_succeed_then_prints_success_message() {
        // Arrange
        let amount_removed = 123usize;
        let expected_success_message = CommandOutputVariant::Value(format!(
            "Effectively removed {} entries (requested {}).",
            amount_removed,
            SOME_TITLES.len()
        ));

        let remove_args = RemoveArgs {
            title: SOME_TITLES.clone(),
        };
        let cli = CheatsheetCli {
            config:  "".to_string(),
            verbose: 0,
            command: Commands::Remove(remove_args.clone()),
        };

        let mut storage = MockEntryStorage::new();
        storage
            .expect_remove()
            .withf(move |entries| entries == &*SOME_TITLES)
            .returning(move |_| Ok(amount_removed));
        let context = CommandContext {
            storage: Box::new(storage),
        };

        // Act
        let result = RemoveCommandImplementation::execute(&context, &cli, &remove_args);

        // Assert
        assert_eq!(expected_success_message, result.unwrap());
    }
}
