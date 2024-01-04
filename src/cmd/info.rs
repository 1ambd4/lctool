use anyhow::Result;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};

use super::Command;
use crate::db;

pub struct InfoCommand;

#[async_trait]
impl Command for InfoCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("info").about("show problem detail").arg(
            Arg::new("id")
                .display_order(1)
                .num_args(1)
                .value_parser(clap::value_parser!(i32))
                .help("problem id"),
        )
    }

    async fn handler(m: &ArgMatches) -> Result<()> {
        let id = *m.get_one::<i32>("id").unwrap();
        let db = db::Sqlite3::global();

        if let Ok(problem) = db.query_with_id(id) {
            println!("{:#?}", problem);
        }

        Ok(())
    }
}
