use std::{
    env::home_dir,
    fs::{create_dir_all, remove_dir_all, remove_file, try_exists, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;

use crate::{config, leetcode::Url};

pub async fn format_json(src: PathBuf, dst: PathBuf) -> Result<()> {
    let cat_command = Command::new("cat")
        .arg(src)
        .stdout(Stdio::piped())
        .spawn()
        .expect("execute `cat` failed");

    let jq_command = Command::new("jq")
        .arg(".")
        .stdin(cat_command.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("execute `jq` failed");

    // `cat leetcode.json | jq . | tee lc.json`
    let mut tee_command = Command::new("tee")
        .arg(dst)
        .stdin(jq_command.stdout.unwrap())
        .stdout(Stdio::null())
        .spawn()
        .expect("execute `tee` failed");

    let status = tee_command.wait().unwrap();
    if !status.success() {
        eprintln!("format json failed!!!");
    }

    Ok(())
}

pub async fn delete_previous_index() -> Result<()> {
    let mut path = home_dir()
        .unwrap()
        .join(".config")
        .join("lctool")
        .join("data");

    if std::fs::try_exists(path.join("raw")).unwrap() {
        remove_dir_all(path.join("raw")).unwrap();
    }

    if std::fs::try_exists(path.join("fmt")).unwrap() {
        remove_dir_all(path.join("fmt")).unwrap();
    }

    path.pop();
    if std::fs::try_exists(path.join("lc.sqlite")).unwrap() {
        remove_file(path.join("lc.sqlite")).unwrap();
    }

    Ok(())
}

pub async fn query_all(path: PathBuf) -> Result<i32> {
    let mut index = 0;

    loop {
        let limit = 100;
        let mut skip = index * limit;

        let query_body = json!({
            "query": vec![
                "query problemsetQuestionList($limit: Int, $skip: Int) {",
                "   problemsetQuestionList(limit: $limit skip: $skip) {",
                "       hasMore",
                "       total",
                "       questions {",
                "           acRate",
                "           difficulty",
                "           title",
                "           titleCn",
                "           titleSlug",
                "       }",
                "   }",
                "}",
            ].join("\n"),
            "variables": {
                "skip": skip,
                "limit": limit,
            },
            "operationName": "problemsetQuestionList"
        });

        let mut headers = HeaderMap::new();
        headers.insert(
            "x-csrftoken",
            HeaderValue::from_str(&config::Config::global().cookies.csrf().unwrap())?,
        );
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("Origin", HeaderValue::from_static("https://leetcode.cn"));

        let client = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) \
                 Chrome/120.0.0.0 Safari/537.36",
            )
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        let resp = client
            // .post("https://leetcode.cn/graphql/")
            .post(Url::global().graphql())
            .json(&query_body)
            .send()
            .await?
            .text()
            .await?;

        let filename = format!("lc-{}.json", index);
        let filepath = path.join(filename);

        println!("write: {:#?}", filepath);

        let mut file = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(filepath)
            .unwrap();
        file.write_all(&resp.as_bytes()).unwrap();

        let resp_json: serde_json::Value = serde_json::from_str(resp.as_str()).unwrap();
        let has_more = &resp_json["data"]["problemsetQuestionList"]["hasMore"];
        if has_more.as_bool().unwrap() {
            index += 1;
        } else {
            break;
        }
    }

    Ok(index)
}

mod test {
    use serde_json::json;

    #[test]
    fn concat_vec_to_json() {
        let query_body = json!({
            "query": vec![
                "query problemsetQuestionList($limit: Int, $skip: Int) {",
                "   problemsetQuestionList(limit: $limit skip: $skip) {",
                "       hasMore",
                "       total",
                "       questions {",
                "           acRate",
                "           difficulty",
                "           title",
                "           titleCn",
                "           titleSlug",
                "       }",
                "   }",
                "}",
            ].join("\n"),
        });

        println!("{:#?}", query_body);
    }
}
