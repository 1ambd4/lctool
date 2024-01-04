use anyhow::Result;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use std::path::PathBuf;

use super::Command;
use crate::{config, db};

pub struct EditCommand;

#[async_trait]
impl Command for EditCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("edit").about("edit problem by id").arg(
            Arg::new("id")
                .num_args(1)
                .required(true)
                .value_parser(clap::value_parser!(i32))
                .help("problem id"),
        )
    }

    async fn handler(m: &ArgMatches) -> Result<()> {
        let id = *m.get_one::<i32>("id").unwrap();
        let db = db::Sqlite3::global();

        if let Ok(problem) = db.query_with_id(id) {
            let filename = format!("{:04}.{}.cpp", problem.id, problem.slug).replace("-", "_");
            let testfile = format!("{:04}.in", problem.id);
            let path = PathBuf::from(config::Config::global().storage.project().unwrap());

            let _ = std::process::Command::new("nvim")
                .arg(path.join(filename))
                .arg(path.join(testfile))
                .status()
                .expect("exec nvim failed");
        }

        Ok(())
    }
}
