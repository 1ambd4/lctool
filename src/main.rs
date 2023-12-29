#![feature(fs_try_exists)]

use std::{env::home_dir, path::PathBuf, process::Command, sync::Arc};

use anyhow::Result;
use rusqlite::Connection;
use tokio::sync::Mutex;

use clap::Parser;

use crate::setting::Settings;

mod cli;
mod db;
mod lc;
mod setting;
mod util;

async fn cache() -> Result<()> {
    util::delete_previous_index().await?;

    let path = Arc::new(
        home_dir()
            .unwrap()
            .join(".config")
            .join("lctool")
            .join("data"),
    );

    std::fs::create_dir_all(path.join("raw"))?;
    std::fs::create_dir_all(path.join("fmt"))?;
    let index = lc::query_all(path.join("raw")).await?;

    let mut formatter_handler = vec![];
    for i in 0..=index {
        let path = Arc::clone(&path);
        formatter_handler.push(tokio::spawn(async move {
            let src = path.join("raw").join(format!("lc-{}.json", i));
            let dst = path.join("fmt").join(format!("lc-{}.json", i));
            util::format_json(src, dst).await.unwrap();
        }));
    }

    for handler in formatter_handler {
        tokio::join!(handler);
    }

    println!("saving to database, please wait...");

    let conn = Arc::new(Mutex::new(Connection::open(
        home_dir()
            .unwrap()
            .join(".config")
            .join("lctool")
            .join("lc.sqlite"),
    )?));

    conn.lock().await.execute(
        "CREATE TABLE leetcode(
            id         INTEGER PRIMARY KEY,
            cn         TEXT,
            en         TEXT,
            slug       TEXT,
            ac_rate    REAL,
            difficulty TEXT
        )",
        (),
    )?;

    let mut saver_handler = vec![];
    for i in 0..=index {
        let path = Arc::clone(&path);
        let conn = Arc::clone(&conn);
        saver_handler.push(tokio::spawn(async move {
            let src = path.join("fmt").join(format!("lc-{}.json", i));
            let conn = Arc::clone(&conn);
            db::save_to_db(src, conn).await.unwrap();
        }));
    }

    for handler in saver_handler {
        tokio::join!(handler);
    }

    Ok(())
}

async fn query(id: i32) -> Result<()> {
    if let Ok(question) = db::query_with_id(id).await {
        println!("{:#?}", question);
    }
    Ok(())
}

async fn write(id: i32) -> Result<()> {
    if let Ok(question) = db::query_with_id(id).await {
        let filename = format!("{:04}.{}.cpp", question.id, question.slug).replace("-", "_");
        let testfile = format!("{:04}.in", question.id);

        // TODO
        // auto parse test data from leetcode
        let _ = Command::new("nvim")
            .arg(PathBuf::from(&Settings::global().path().unwrap()).join(filename))
            .arg(PathBuf::from(&Settings::global().path().unwrap()).join(testfile))
            .status()
            .expect("exec nvim failed");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    match &cli.command {
        cli::Commands::Cache => {
            cache().await?;
        }
        cli::Commands::Today => {
            lc::query_today().await?;
        }
        cli::Commands::Query(args) => {
            query(args.id).await?;
        }
        cli::Commands::Write(args) => {
            write(args.id).await?;
        }
        cli::Commands::Info => {
            let setting = setting::Settings::global();
            println!("{:?}", setting.path());
        }
    }

    Ok(())
}
