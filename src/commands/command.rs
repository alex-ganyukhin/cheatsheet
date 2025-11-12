/// Context for commands execution. Holds all necessary dependencies, configurations, etc
pub struct CommandContext {
    pub storage: Box<dyn crate::domain::entry_storage::EntryStorage>,
    pub writer: Box<dyn std::io::Write>,
}

/// Abstraction for a command that can be executed in the application.
pub trait CommandImplementation {
    fn execute(context: &mut CommandContext, cli: &crate::cli::CheatsheetCli) -> Result<(), anyhow::Error>;
}

pub struct NotImplementedCommandImplementation;

impl CommandImplementation for NotImplementedCommandImplementation {
    fn execute(_context: &mut CommandContext, _cli: &crate::cli::CheatsheetCli) -> Result<(), anyhow::Error> {
        spdlog::warn!(
            "The command {} is not implemented yet, will be supported in future releases.",
            _cli.command.as_ref()
        );

        Err(anyhow::anyhow!("Not implemented yet"))
    }
}
