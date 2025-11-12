pub mod command;
pub mod list;
pub mod search;
pub mod show_config;

pub type SearchCommandImplementation = command::NotImplementedCommandImplementation;
pub type AddCommandImplementation = command::NotImplementedCommandImplementation;
pub type RemoveCommandImplementation = command::NotImplementedCommandImplementation;

pub use command::CommandImplementation;
pub use list::ListCommandImplementation;
pub use show_config::ShowConfigCommandImplementation;
