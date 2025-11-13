use crate::commands::{command::CommandContext, CommandImplementation, CommandOutputVariant};

pub struct ShowConfigCommandImplementation;

impl CommandImplementation for ShowConfigCommandImplementation {
    fn execute(
        _context: &CommandContext,
        cli: &crate::cli::CheatsheetCli,
    ) -> Result<CommandOutputVariant, anyhow::Error> {
        debug_assert!(
            matches!(cli.command, crate::cli::Commands::ShowConfig(_)),
            "Expected ShowConfig command"
        );

        Ok(CommandOutputVariant::Value(cli.config.clone()))
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::cli::CheatsheetCli;
    use crate::commands::command::CommandContext;
    use crate::commands::command::CommandImplementation;
    use crate::commands::show_config::ShowConfigCommandImplementation;
    use crate::commands::CommandOutputVariant;
    use crate::domain::entry_storage::MockEntryStorage;

    const SOME_TOML_CONFIG_PATH: &str = "path/to/storage.toml";
    const DUMMY_BINARY_NAME: &str = "cheatsheet";

    #[test]
    fn test_show_config_command() {
        // Arrange
        let expected_output = CommandOutputVariant::Value(SOME_TOML_CONFIG_PATH.to_string());
        let cli = CheatsheetCli::parse_from(vec![DUMMY_BINARY_NAME, "-c", SOME_TOML_CONFIG_PATH, "show-config"].iter());

        let storage = MockEntryStorage::new();
        let context = CommandContext {
            storage: Box::new(storage),
        };

        // Act
        let result = ShowConfigCommandImplementation::execute(&context, &cli);

        // Assert
        assert_eq!(expected_output, result.unwrap());
    }
}
