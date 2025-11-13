use indexmap::IndexMap;

/// Context for commands execution. Holds all necessary dependencies, configurations, etc
pub struct CommandContext {
    pub storage: Box<dyn crate::domain::entry_storage::EntryStorage>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// The variant of output which may be produced by a command.
///
/// For the time being, only string values are supported for simplicity.
///
#[derive(PartialEq, Clone, Debug)]
pub enum CommandOutputVariant {
    Value(String),
    Array(Vec<CommandOutputVariant>),
    Dict(IndexMap<String, CommandOutputVariant>),
}

/// Abstraction for processing command outputs.
///
/// The concrete implementation may format output in different ways, output into different destinations
/// (console, file, network, etc).
pub trait CommandOutputProcessor {
    fn process_output(&mut self, output: &CommandOutputVariant);
}

/// Abstraction for processing command errors.
///
/// The concrete implementation may format errors in different ways, output into different destinations
/// (console, file, network, etc).
pub trait CommandErrorProcessor {
    fn process_error(&mut self, error: &anyhow::Error);
}

pub trait CommandOutputAndErrorProcessor: CommandOutputProcessor + CommandErrorProcessor {}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// Abstraction for a command that can be executed in the application.
pub trait CommandImplementation {
    fn execute(
        context: &CommandContext,
        cli: &crate::cli::CheatsheetCli,
    ) -> Result<CommandOutputVariant, anyhow::Error>;
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct NotImplementedCommandImplementation;

impl CommandImplementation for NotImplementedCommandImplementation {
    fn execute(
        _context: &CommandContext,
        _cli: &crate::cli::CheatsheetCli,
    ) -> Result<CommandOutputVariant, anyhow::Error> {
        spdlog::warn!(
            "The command {} is not implemented yet, will be supported in future releases.",
            _cli.command.as_ref()
        );

        Err(anyhow::anyhow!("Not implemented yet"))
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
