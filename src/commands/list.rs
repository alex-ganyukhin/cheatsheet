use crate::commands::{command::CommandContext, CommandImplementation};

pub struct ListCommandImplementation;

impl CommandImplementation for ListCommandImplementation {
    fn execute(context: &CommandContext, _cli: &crate::cli::CheatsheetCli) -> Result<(), anyhow::Error> {
        context.storage.load_all().map(|entries| {
            for entry in entries {
                println!(
                    "Title: {}\nCommand: {}\nDescription: {}\n",
                    entry.title,
                    entry.command,
                    entry.description.unwrap_or_default()
                );
            }
        })
    }
}
