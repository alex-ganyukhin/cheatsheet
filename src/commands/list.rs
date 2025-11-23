use crate::{
    cli::{CheatsheetCli, ListArgs},
    commands::{command::CommandContext, CommandImplementation, CommandOutputVariant},
};


pub struct ListCommandImplementation;


impl CommandImplementation<ListArgs> for ListCommandImplementation {
    fn execute(
        context: &CommandContext,
        _cli: &CheatsheetCli,
        _command_config: &ListArgs,
    ) -> Result<CommandOutputVariant, anyhow::Error> {
        let entries = context.storage.load_all()?;


        let printable_entries = entries
            .into_iter()
            .map(move |entry| entry.into())
            .collect::<Vec<CommandOutputVariant>>();


        return Ok(CommandOutputVariant::Array(printable_entries));
    }
}
