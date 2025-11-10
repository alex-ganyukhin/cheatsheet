use cheatsheet::cli::CheatsheetCli;
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

fn main() {
    let cli = CheatsheetCli::parse();

    setup_logger(&cli);

    spdlog::debug!("Parsed CLI arguments: {:#?}", cli);

    match cli.command {
        cheatsheet::cli::Commands::Search(args) => {
            spdlog::warn!(
                "Executing 'search' command with args: {:#?} \n\nIS NOT IMPLEMENTED YET.",
                args
            );
        }
        cheatsheet::cli::Commands::List(args) => {
            spdlog::warn!(
                "Executing 'list' command with args: {:#?} \n\nIS NOT IMPLEMENTED YET.",
                args
            );
        }
        cheatsheet::cli::Commands::Add(args) => {
            spdlog::warn!(
                "Executing 'add' command with args: {:#?} \n\nIS NOT IMPLEMENTED YET.",
                args
            );
        }
        cheatsheet::cli::Commands::Remove(args) => {
            spdlog::warn!(
                "Executing 'remove' command with args: {:#?} \n\nIS NOT IMPLEMENTED YET.",
                args
            );
        }
        cheatsheet::cli::Commands::ShowConfig(_) => {
            println!("Using config path: {}", cli.config.as_deref().unwrap_or("UNKNOWN"));
        }
    }
}
