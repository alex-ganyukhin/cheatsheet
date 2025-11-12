use crate::commands::{command::CommandContext, CommandImplementation};

pub struct ShowConfigCommandImplementation;

impl CommandImplementation for ShowConfigCommandImplementation {
    fn execute(_context: &mut CommandContext, cli: &crate::cli::CheatsheetCli) -> Result<(), anyhow::Error> {
        debug_assert!(
            matches!(cli.command, crate::cli::Commands::ShowConfig(_)),
            "Expected ShowConfig command"
        );

        writeln!(_context.writer, "{}", cli.config)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use mockall::PredicateStrExt;

    use crate::cli::CheatsheetCli;
    use crate::commands::command::CommandContext;
    use crate::commands::command::CommandImplementation;
    use crate::commands::show_config::ShowConfigCommandImplementation;
    use crate::domain::entry_storage::MockEntryStorage;

    use mockall;
    use mockall::predicate::*;

    const SOME_TOML_CONFIG_PATH: &str = "path/to/storage.toml";
    const DUMMY_BINARY_NAME: &str = "cheatsheet";

    mockall::mock! {
        #[derive(Debug)]
        pub WriterMock{}

        impl std::io::Write for WriterMock {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
            fn flush(&mut self) -> std::io::Result<()>;
        }
    }

    #[test]
    fn test_show_config_command() {
        // Arrange
        let cli = CheatsheetCli::parse_from(vec![DUMMY_BINARY_NAME, "-c", SOME_TOML_CONFIG_PATH, "show-config"].iter());

        let storage = MockEntryStorage::new();
        let mut writer = MockWriterMock::new();
        writer
            .expect_write()
            .with(eq(SOME_TOML_CONFIG_PATH).from_utf8())
            .return_once(move |_| Ok(SOME_TOML_CONFIG_PATH.len()));
        writer
            .expect_write()
            .with(eq("\n".as_bytes()))
            .return_once(move |_| Ok(1));

        let mut context = CommandContext {
            storage: Box::new(storage),
            writer: Box::new(writer),
        };

        // Act
        let result = ShowConfigCommandImplementation::execute(&mut context, &cli);

        // Assert
        assert!(result.is_ok());
    }
}
