use anyhow::Result;
use rusqlite::Connection;
use std::{fs::File, io::BufReader, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use self::sqlite::INSERT_RECORD_OR_UPDATE_IF_EXISTS;

mod sqlite;
pub use sqlite::Sqlite3;

pub async fn save_to_db(src: PathBuf, conn: Arc<Mutex<Connection>>) -> Result<()> {
    let reader = BufReader::new(File::options().read(true).open(src)?);
    let leetcode: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let questions: &serde_json::Value = &leetcode["data"]["problemsetQuestionList"]["questions"];

    for i in 0..1000 {
        if let Some(question) = questions.get(i) {
            if let Ok(id) = question["frontendQuestionId"]
                .as_str()
                .unwrap()
                .parse::<i32>()
            {
                let cn = question["titleCn"].as_str().unwrap();
                let en = question["title"].as_str().unwrap();
                let slug = question["titleSlug"].as_str().unwrap();
                let rate = question["acRate"].as_f64().unwrap();
                let level = question["difficulty"].as_str().unwrap();

                conn.lock()
                    .await
                    .execute(
                        // "INSERT INTO leetcode
                        // (id, cn, en, slug, rate, level)
                        // VALUES
                        // (?1, ?2, ?3, ?4, ?5, ?6)",
                        INSERT_RECORD_OR_UPDATE_IF_EXISTS,
                        (id, cn, en, slug, rate, level),
                    )
                    .unwrap();
            }
        }
    }

    Ok(())
}
