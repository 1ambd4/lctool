use anyhow::Result;
use async_trait::async_trait;
use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};

use super::Command;
use crate::cache;

pub struct DataCommand;

#[async_trait]
impl Command for DataCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("data")
            .about("manage cache")
            .arg(
                Arg::new("cache")
                    .display_order(1)
                    .long("cache")
                    .help("cache data")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("update")
                    .display_order(2)
                    .long("update")
                    .help("update index")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("delete")
                    .display_order(3)
                    .long("delete")
                    .help("delete cache")
                    .action(ArgAction::SetTrue),
            )
    }

    async fn handler(m: &ArgMatches) -> Result<()> {
        if m.get_flag("cache") {
            cache::cache().await?;
        }

        if m.get_flag("update") {
            cache::update().await?;
        }

        if m.get_flag("delete") {
            todo!()
        }

        Ok(())
    }
}
