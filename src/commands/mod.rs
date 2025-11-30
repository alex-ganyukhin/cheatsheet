pub mod add;
pub mod command;
pub mod list;
pub mod plaintext_command_output_processor;
pub mod remove;
pub mod search;
pub mod show_config;


pub use add::AddCommandImplementation;
pub use list::ListCommandImplementation;
pub use remove::RemoveCommandImplementation;
pub use search::SearchCommandImplementation;
pub use show_config::ShowConfigCommandImplementation;

pub use command::CommandErrorProcessor;
pub use command::CommandImplementation;
pub use command::CommandOutputProcessor;
pub use command::CommandOutputVariant;

pub use plaintext_command_output_processor::PlaintextCommandOutputAndErrorProcessor;
