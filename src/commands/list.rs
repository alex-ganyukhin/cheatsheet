use indexmap::IndexMap;

use crate::commands::{command::CommandContext, CommandImplementation, CommandOutputVariant};

pub struct ListCommandImplementation;

impl CommandImplementation for ListCommandImplementation {
    fn execute(
        context: &CommandContext,
        _cli: &crate::cli::CheatsheetCli,
    ) -> Result<CommandOutputVariant, anyhow::Error> {
        let entries = context.storage.load_all()?;

        let printable_entries = entries
            .iter()
            .map(move |entry| {
                CommandOutputVariant::Dict(IndexMap::from([
                    ("title".to_string(), CommandOutputVariant::Value(entry.title.clone())),
                    (
                        "command".to_string(),
                        CommandOutputVariant::Value(entry.command.clone()),
                    ),
                    (
                        "description".to_string(),
                        CommandOutputVariant::Value(entry.description.clone().unwrap_or_default()),
                    ),
                ]))
            })
            .collect::<Vec<CommandOutputVariant>>();

        return Ok(CommandOutputVariant::Array(printable_entries));
    }
}
