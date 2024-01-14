use anyhow::Result;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use log::{debug, info};
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

use super::Command;
use crate::{config, db, leetcode};

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

            if !path.join(&testfile).exists() {
                let testcase = leetcode::Request::default()
                    .query_testcase_with_id(id)
                    .await?
                    .replace("\"", "")
                    .replace("\\n", "\n")
                    .replace(",", " ")
                    .replace("[", "")
                    .replace("]", "");

                let mut file = File::options()
                    .create(true)
                    .append(true)
                    .open(path.join(&testfile))
                    .unwrap();
                file.write_all(testcase.as_bytes()).unwrap();
                file.flush().unwrap();

                debug!("{:#?} not exists, would create it.", file);
                debug!("testcase: {:#?}", testcase);
            }

            let now = Instant::now();

            let _ = std::process::Command::new("nvim")
                .arg(path.join(&filename))
                .arg(path.join(&testfile))
                .status()
                .expect("exec nvim failed");

            let elapsed_time = now.elapsed();
            let minute = elapsed_time.as_secs() / 60;
            let second = elapsed_time - Duration::from_secs(minute * 60);
            info!("use time: {}:{}", minute, second.as_secs());
        }

        Ok(())
    }
}
