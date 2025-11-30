use clap::ArgAction;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;


use crate::constants;


#[derive(Parser, Debug)]
#[command(name = "cheatsheet", version, about = "A command-line cheatsheet manager")]
pub struct CheatsheetCli {
    #[arg(short, long, help = "Path to the configuration file", env = constants::env_vars::CONFIG_PATH, default_value = constants::defaults::CONFIG_PATH.as_str())]
    pub config: String,

    #[arg(short, long, action = ArgAction::Count, help = "Enable verbose output")]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand, Debug, strum_macros::AsRefStr)]
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
#[derive(Parser, Debug, Clone)]
pub struct SearchArgs {
    #[arg(short, long, value_enum, default_value_t = SearchType::Fuzzy, help = "The type of search to perform.")]
    pub search_type: SearchType,

    #[arg(
        short,
        long,
        help = "The maximum number of results to return.",
        long_help = "The maximum number of results to return. If not specified, all matching results will be returned."
    )]
    pub limit: Option<usize>,

    #[arg(
        short,
        long,
        help = "Display all fields of the matched entries.",
        long_help = "Display all fields of the matched entries. If not set, only the command itself will be displayed."
    )]
    pub full: bool,

    #[arg(required = true, trailing_var_arg = true, help = "The search query itself.")]
    pub query: Vec<String>,
}


#[derive(Debug, Clone, ValueEnum, strum_macros::AsRefStr)]
pub enum SearchType {
    Exact,
    Fuzzy,
    Substr,
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
#[derive(Parser, Debug, Clone)]
pub struct AddArgs {
    #[arg(short, long, required = true, help = "Title of the command. Must be unique.")]
    pub title: String,

    #[arg(short, long, required = true, help = "The command itself")]
    pub command: String,

    #[arg(short, long, required = false, help = "Description of the command")]
    pub description: Option<String>,

    #[arg(
        short,
        long,
        help = "If set, if an entry with the same title exists, it will be replaced, otherwise it will be appended."
    )]
    pub replacing: bool,
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


/// The "remove" sub-command
/// - Removes a command from the cheatsheet by its title
#[derive(Parser, Debug)]
pub struct RemoveArgs {
    #[arg(short, long, required = true, help = "Title of the command to remove.")]
    pub title: String,
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


/// The "show-config" sub-command
/// - Prints the effective configuration of the application, e.g. path to the use config file (cheatsheet.toml)
#[derive(Parser, Debug, Clone, Copy)]
pub struct ShowConfigArgs {}
