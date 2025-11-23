use crate::{
    cli::{CheatsheetCli, ShowConfigArgs},
    commands::{command::CommandContext, CommandImplementation, CommandOutputVariant},
};

pub struct ShowConfigCommandImplementation;

impl CommandImplementation<ShowConfigArgs> for ShowConfigCommandImplementation {
    fn execute(
        _context: &CommandContext,
        cli: &CheatsheetCli,
        _config_args: &ShowConfigArgs,
    ) -> Result<CommandOutputVariant, anyhow::Error> {
        Ok(cli.config.clone().into())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(test)]

mod tests {

    use crate::cli::CheatsheetCli;
    use crate::cli::Commands;
    use crate::cli::ShowConfigArgs;
    use crate::commands::command::CommandContext;
    use crate::commands::command::CommandImplementation;
    use crate::commands::show_config::ShowConfigCommandImplementation;
    use crate::commands::CommandOutputVariant;
    use crate::domain::entry_storage::MockEntryStorage;

    const SOME_TOML_CONFIG_PATH: &str = "path/to/storage.toml";


    #[test]
    fn test_show_config_command_when_invoked_then_outputs_effective_config_path() {
        // Arrange
        let expected_output = CommandOutputVariant::Value(SOME_TOML_CONFIG_PATH.to_string());

        let show_config_args = ShowConfigArgs {};

        let cli = CheatsheetCli {
            config:  SOME_TOML_CONFIG_PATH.to_string(),
            verbose: 0,
            command: Commands::ShowConfig(show_config_args),
        };

        let storage = MockEntryStorage::new();

        let context = CommandContext {
            storage: Box::new(storage),
        };

        // Act
        let result = ShowConfigCommandImplementation::execute(&context, &cli, &show_config_args);

        // Assert
        assert_eq!(expected_output, result.unwrap());
    }
}
