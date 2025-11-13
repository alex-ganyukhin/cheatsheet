pub mod command;
pub mod list;
pub mod plaintext_command_output_processor;
pub mod search;
pub mod show_config;

pub type SearchCommandImplementation = command::NotImplementedCommandImplementation;
pub type AddCommandImplementation = command::NotImplementedCommandImplementation;
pub type RemoveCommandImplementation = command::NotImplementedCommandImplementation;

pub use command::CommandErrorProcessor;
pub use command::CommandImplementation;
pub use command::CommandOutputProcessor;
pub use command::CommandOutputVariant;
pub use list::ListCommandImplementation;
pub use plaintext_command_output_processor::PlaintextCommandOutputAndErrorProcessor;
pub use show_config::ShowConfigCommandImplementation;
