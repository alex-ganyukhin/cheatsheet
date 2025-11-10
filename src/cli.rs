use clap::ArgAction;
use clap::Parser;
use clap::Subcommand;

use crate::constants;

#[derive(Parser, Debug)]
#[command(name = "cheatsheet", version, about = "A command-line cheatsheet manager")]
pub struct CheatsheetCli {
    #[arg(short, long, help = "Path to the configuration file", env = constants::env_vars::CONFIG_PATH, default_value = "")]
    pub config: Option<String>,

    #[arg(short, long, action = ArgAction::Count, help = "Enable verbose output")]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Search(SearchArgs),
    ShowConfig(ShowConfigArgs),
    Add(AddArgs),
    Remove(RemoveArgs),
    List(ListArgs),
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// The "search" sub-command, the primary command of the application
/// - Searches the cheatsheet for commands matching the query
#[derive(Parser, Debug)]
pub struct SearchArgs {
    #[arg(short, long, help = "Perform an exact match search.")]
    exact: bool,

    /// The search query
    #[arg(required = true, trailing_var_arg = true, help = "The search query itself.")]
    query: Vec<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// The "list" sub-command
/// - Lists all commands in the cheatsheet
#[derive(Parser, Debug)]
pub struct ListArgs {}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// The "add" sub-command
/// - Adds(appends) a new command to the cheatsheet
/// - Requires title, command, and optional description
#[derive(Parser, Debug)]
pub struct AddArgs {
    #[arg(short, long, required = true, help = "Title of the command. Must be unique.")]
    title: String,

    #[arg(short, long, required = true, help = "The command itself")]
    command: String,

    #[arg(short, long, required = false, help = "Description of the command")]
    description: String,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// The "remove" sub-command
/// - Removes a command from the cheatsheet by its title
#[derive(Parser, Debug)]
pub struct RemoveArgs {
    #[arg(short, long, required = true, help = "Title of the command to remove.")]
    title: String,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// The "show-config" sub-command
/// - Prints the effective configuration of the application, e.g. path to the use config file (cheatsheet.toml)
#[derive(Parser, Debug)]
pub struct ShowConfigArgs {}
