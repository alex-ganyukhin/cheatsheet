pub mod add;
pub mod command;
pub mod list;
pub mod plaintext_command_output_processor;
pub mod search;
pub mod show_config;


pub type RemoveCommandImplementation = command::NotImplementedCommandImplementation;


pub use add::AddCommandImplementation;
pub use command::CommandErrorProcessor;
pub use command::CommandImplementation;
pub use command::CommandOutputProcessor;
pub use command::CommandOutputVariant;
pub use list::ListCommandImplementation;
pub use plaintext_command_output_processor::PlaintextCommandOutputAndErrorProcessor;
pub use search::SearchCommandImplementation;
pub use show_config::ShowConfigCommandImplementation;
