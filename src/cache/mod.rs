use anyhow::Result;
use rusqlite::Connection;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use crate::{config, db, util};

pub async fn cache() -> Result<()> {
    // crate::util::delete_previous_index().await?;

    let path =
        Arc::new(PathBuf::from(config::Config::global().storage.root().unwrap()).join("temp"));

    std::fs::create_dir_all(path.join("raw"))?;
    std::fs::create_dir_all(path.join("fmt"))?;
    let index = util::query_all(path.join("raw")).await?;

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

    Ok(())
}

pub async fn update() -> Result<()> {
    println!("update index to database, please wait...");

    let path =
        Arc::new(PathBuf::from(config::Config::global().storage.root().unwrap()).join("temp"));

    let conn = Arc::new(Mutex::new(Connection::open(
        config::Config::global().storage.cache().unwrap(),
    )?));

    conn.lock().await.execute(
        "CREATE TABLE IF NOT EXISTS leetcode(
            id         INTEGER PRIMARY KEY,
            cn         TEXT,
            en         TEXT,
            slug       TEXT,
            rate       REAL,
            level      TEXT
        )",
        (),
    )?;

    // TODO
    let index = 33;

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
