use anyhow::Result;
use async_trait::async_trait;
use clap::{ArgMatches, Command as ClapCommand};

#[async_trait]
pub trait Command {
    fn usage() -> ClapCommand;
    async fn handler(m: &ArgMatches) -> Result<()>;
}

mod data;
mod edit;
mod info;
mod list;

pub use data::DataCommand;
pub use edit::EditCommand;
pub use info::InfoCommand;
pub use list::ListCommand;
