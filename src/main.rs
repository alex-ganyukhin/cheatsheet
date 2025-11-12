use cheatsheet::commands::command::CommandContext;
use cheatsheet::commands::AddCommandImplementation;
use cheatsheet::commands::CommandImplementation;
use cheatsheet::commands::ListCommandImplementation;
use cheatsheet::commands::RemoveCommandImplementation;
use cheatsheet::commands::SearchCommandImplementation;
use cheatsheet::commands::ShowConfigCommandImplementation;

use cheatsheet::storage::TomlEntryStorage;

use cheatsheet::cli::{CheatsheetCli, Commands};

use clap::Parser;

fn setup_logger(cli: &CheatsheetCli) {
    let logger = spdlog::default_logger();

    match cli.verbose {
        0 => logger.set_level_filter(spdlog::LevelFilter::Off),
        1 => {
            logger.set_level_filter(spdlog::LevelFilter::MoreSevereEqual(spdlog::Level::Info));
            spdlog::info!("Verbose mode enabled");
        }
        2 => {
            logger.set_level_filter(spdlog::LevelFilter::MoreSevereEqual(spdlog::Level::Debug));
            spdlog::debug!("Very verbose(debug) mode enabled");
        }
        _ => {
            logger.set_level_filter(spdlog::LevelFilter::MoreSevereEqual(spdlog::Level::Trace));
            spdlog::trace!("Very very verbose(trace) mode enabled");
        }
    };
}

fn load_app_context(cli: &CheatsheetCli) -> CommandContext {
    CommandContext {
        storage: Box::new(TomlEntryStorage::new(std::path::PathBuf::from(cli.config.clone()))),
    }
}

fn log_error(err: &anyhow::Error) {
    spdlog::error!("Error: {}", err);

    let mut source = err.source();
    while let Some(inner) = source {
        spdlog::error!("Caused by: {}", inner);
        source = inner.source();
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cli = CheatsheetCli::parse();

    setup_logger(&cli);

    spdlog::debug!("Parsed CLI arguments: {:#?}", cli);

    let context = load_app_context(&cli);

    #[cfg_attr(any(), rustfmt::skip)]
    match &cli.command {
        Commands::Search(_)     => SearchCommandImplementation::execute(&context, &cli),
        Commands::Add(_)        => AddCommandImplementation::execute(&context, &cli),
        Commands::Remove(_)     => RemoveCommandImplementation::execute(&context, &cli),
        Commands::List(_)       => ListCommandImplementation::execute(&context, &cli),
        Commands::ShowConfig(_) => ShowConfigCommandImplementation::execute(&context, &cli),
    }.inspect_err(log_error)
}
