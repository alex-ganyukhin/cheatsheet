use cheatsheet::commands::command::CommandContext;
use cheatsheet::commands::AddCommandImplementation;
use cheatsheet::commands::CommandImplementation;
use cheatsheet::commands::ListCommandImplementation;
use cheatsheet::commands::PlaintextCommandOutputAndErrorProcessor;
use cheatsheet::commands::RemoveCommandImplementation;
use cheatsheet::commands::SearchCommandImplementation;
use cheatsheet::commands::ShowConfigCommandImplementation;

use cheatsheet::commands::command::CommandOutputAndErrorProcessor;
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

fn load_output_processor(_cli: &CheatsheetCli) -> Box<dyn CommandOutputAndErrorProcessor> {
    // For the time being, we only have one output processor (console), so we return it directly.
    Box::new(PlaintextCommandOutputAndErrorProcessor::default())
}

fn main() -> Result<(), anyhow::Error> {
    let cli = CheatsheetCli::parse();

    setup_logger(&cli);

    spdlog::debug!("Parsed CLI arguments: {:#?}", cli);

    let mut context = load_app_context(&cli);
    let mut output_processor = load_output_processor(&cli);

    #[cfg_attr(any(), rustfmt::skip)]
    match &cli.command {
        Commands::Search(_)     => SearchCommandImplementation::execute(& mut context, &cli),
        Commands::Add(_)        => AddCommandImplementation::execute(& mut context, &cli),
        Commands::Remove(_)     => RemoveCommandImplementation::execute(& mut context, &cli),
        Commands::List(_)       => ListCommandImplementation::execute(& mut context, &cli),
        Commands::ShowConfig(_) => ShowConfigCommandImplementation::execute(& mut context, &cli),
    }
    .inspect( |v|output_processor.as_mut().process_output(v))
    .inspect_err( |e|output_processor.as_mut().process_error(e))
    .map(|_| ())
}
