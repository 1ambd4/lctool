use std::{
    cell::OnceCell,
    env::home_dir,
    fs::{try_exists, File},
    io::BufReader,
    path::PathBuf,
    sync::Arc,
};

use anyhow::Result;
use rusqlite::Connection;
use tokio::sync::Mutex;

use crate::lc::Question;

pub async fn query_with_id(id: i32) -> Result<Question, anyhow::Error> {
    let db_path = home_dir()
        .unwrap()
        .join(".config")
        .join("lctool")
        .join("lc.sqlite");

    if !try_exists(db_path).unwrap() {
        crate::cache().await?;
    }

    let conn = &Db::global().conn;
    let mut stmt = conn.prepare("SELECT * FROM leetcode where id = ?1")?;
    let mut rows = stmt.query(rusqlite::params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Question {
            id: row.get(0)?,
            cn: row.get(1)?,
            en: row.get(2)?,
            slug: row.get(3)?,
            ac_rate: row.get(4)?,
            difficulty: row.get(5)?,
        })
    } else {
        unreachable!()
    }
}

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
                let ac_rate = question["acRate"].as_f64().unwrap();
                let difficulty = question["difficulty"].as_str().unwrap();

                conn.lock()
                    .await
                    .execute(
                        "INSERT INTO leetcode (
                        id, cn, en, slug, ac_rate, difficulty
                    )
                    VALUES
                    (?1, ?2, ?3, ?4, ?5, ?6)",
                        (id, cn, en, slug, ac_rate, difficulty),
                    )
                    .unwrap();
            }
        }
    }

    Ok(())
}

pub static mut DB: OnceCell<Db> = OnceCell::new();

struct Db {
    conn: Connection,
}

impl Db {
    pub fn global() -> &'static Self {
        let db_path = home_dir()
            .unwrap()
            .join(".config")
            .join("lctool")
            .join("lc.sqlite");

        unsafe {
            DB.get_or_init(|| Db {
                conn: Connection::open(&db_path).unwrap(),
            })
        }
    }
}
