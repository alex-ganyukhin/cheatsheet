use indexmap::IndexMap;


use crate::{cli::CheatsheetCli, domain::Entry};


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


impl From<&str> for CommandOutputVariant {
    fn from(value: &str) -> Self {
        CommandOutputVariant::Value(value.to_string())
    }
}


impl From<String> for CommandOutputVariant {
    fn from(value: String) -> Self {
        CommandOutputVariant::Value(value)
    }
}


impl From<Entry> for CommandOutputVariant {
    fn from(entry: Entry) -> Self {
        CommandOutputVariant::Dict(indexmap::indexmap! {
            "title".to_string() => entry.title.into(),
            "command".to_string() => entry.command.into(),
            "description".to_string() => entry.description.unwrap_or_default().into(),
        })
    }
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


/// Abstraction for a command that can be executed in the application.


pub trait CommandImplementation<CommandConfig = ()> {
    fn execute(
        context: &CommandContext,
        cli: &CheatsheetCli,
        command_config: &CommandConfig,
    ) -> Result<CommandOutputVariant, anyhow::Error>;
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


pub struct NotImplementedCommandImplementation;


impl<CommandConfig> CommandImplementation<CommandConfig> for NotImplementedCommandImplementation {
    fn execute(
        _context: &CommandContext,
        cli: &CheatsheetCli,
        _command_config: &CommandConfig,
    ) -> Result<CommandOutputVariant, anyhow::Error> {
        spdlog::warn!(
            "The command {} is not implemented yet, will be supported in future releases.",
            cli.command.as_ref()
        );


        Err(anyhow::anyhow!("Not implemented yet"))
    }
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
