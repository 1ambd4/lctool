use anyhow::Result;
use rusqlite::Connection;
use std::cell::OnceCell;

pub static QUERY_PROBLEM_DETAIL_WITH_ID: &str = r#"
    SELECT * FROM leetcode WHERE id = ?1
"#;

pub static INSERT_RECORD_OR_UPDATE_IF_EXISTS: &str = r#"
    INSERT INTO leetcode
    (id, cn, en, slug, rate, level)
    VALUES
    (?1, ?2, ?3, ?4, ?5, ?6)
    ON CONFLICT(id) DO UPDATE SET
    cn = ?2, en = ?3, slug = ?4, rate = ?5, level = ?6
"#;

use crate::{config::Config, leetcode::Problem};

pub static mut SQLITE3: OnceCell<Sqlite3> = OnceCell::new();

pub struct Sqlite3 {
    conn: Connection,
}

impl Sqlite3 {
    pub fn global() -> &'static Self {
        let db = Config::global().storage.cache().unwrap();

        unsafe {
            SQLITE3.get_or_init(|| Sqlite3 {
                conn: Connection::open(&db).unwrap(),
            })
        }
    }

    pub fn query_with_id(&self, id: i32) -> Result<Problem> {
        let conn = &Self::global().conn;
        let mut stmt = conn.prepare(QUERY_PROBLEM_DETAIL_WITH_ID)?;
        let mut rows = stmt.query(rusqlite::params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Problem {
                id: row.get(0)?,
                cn: row.get(1)?,
                en: row.get(2)?,
                slug: row.get(3)?,
                rate: row.get(4)?,
                level: row.get(5)?,
            })
        } else {
            unreachable!()
        }
    }

    pub async fn query_with_range(&self, from: i32, to: i32) -> Result<Problem> {
        todo!()
    }
}

mod test {
    use super::{Sqlite3, INSERT_RECORD_OR_UPDATE_IF_EXISTS};
    use tokio::runtime::Builder;

    #[test]
    fn query_with_valid_id_1() {
        let db = Sqlite3::global();
        let id = 1;

        // if let Ok(problem) = Builder::new_multi_thread()
        //     .enable_all()
        //     .build()
        //     .expect("tokio runtime build failed")
        //     .block_on(db.query_with_id(1))
        // {
        //     println!("{:#?}", problem);
        // }

        if let Ok(problem) = db.query_with_id(id) {
            println!("{:#?}", problem);
        }
    }

    // It seems that the tests are run asynchoronously,
    // so those two test will fail due to data race.
    #[test]
    fn query_with_invalid_id_n1() {
        // let db = Sqlite3::global();
    }

    // "INSERT INTO leetcode
    // (id, cn, en, slug, rate, level)
    // VALUES
    // (?1, ?2, ?3, ?4, ?5, ?6)",
    // (id, cn, en, slug, rate, level),

    #[test]
    fn update_value_if_record_exists() {
        let db = &Sqlite3::global().conn;
        db.execute(
            INSERT_RECORD_OR_UPDATE_IF_EXISTS,
            (5000, "b", "b", "b", 0.1, "a"),
        )
        .unwrap();
    }
}
