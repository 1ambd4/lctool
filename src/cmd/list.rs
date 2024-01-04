use anyhow::Result;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};

use super::Command;

pub struct ListCommand;

#[async_trait]
impl Command for ListCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("list").about("list problems").arg(
            Arg::new("range")
                .long("range")
                .num_args(2)
                .value_parser(clap::value_parser!(i32))
                .help("list problems by id range"),
        )
    }

    async fn handler(m: &ArgMatches) -> Result<()> {
        todo!()
    }
}
