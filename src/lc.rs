use std::{fs::File, io::Write, path::PathBuf};

use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;

#[derive(Debug)]
pub struct Question {
    pub id: i32,
    pub cn: String,
    pub en: String,
    pub slug: String,
    pub ac_rate: f32,
    pub difficulty: String,
}

/*
{
    "query": "\n    query questionOfToday {\n  todayRecord {\n    date\n   question {\n      questionId\n      frontendQuestionId: questionFrontendId\n      difficulty\n      title\n      titleCn: translatedTitle\n      titleSlug\n      acRate\n     }\n    }\n}\n    ",
    "operationName": "questionOfToday"
}
*/
pub async fn query_today() -> Result<()> {
    let query_body = json!({
        "query": "\n query questionOfToday {\n todayRecord {\n date\n question {\n questionId\n frontendQuestionId: questionFrontendId\n difficulty\n title\n titleCn: translatedTitle\n titleSlug\n acRate\n }\n}\n}\n",
        "operationName": "questionOfToday"
    });

    let mut headers = HeaderMap::new();
    headers.insert(
        "x-csrftoken",
        HeaderValue::from_static(
            "ICuvUTQIutzPLVKDIhUTHk3aGITDbgfsKDMx600p5VeqnLrcB9QS0bPB3ANX0ulS",
        ),
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
        .post("https://leetcode.cn/graphql/")
        .json(&query_body)
        .send()
        .await?
        .text()
        .await?;

    let resp_json: serde_json::Value = serde_json::from_str(resp.as_str()).unwrap();

    let today_record: &serde_json::Value = &resp_json["data"]["todayRecord"].get(0).unwrap();

    let date = &today_record["date"].as_str().unwrap();
    let question = today_record["question"].as_object().unwrap();
    let id = question["questionId"].as_str().unwrap();
    let ac_rate = question["acRate"].as_number().unwrap();
    let title = question["title"].as_str().unwrap();
    let title_cn = question["title"].as_str().unwrap();

    let output = json!({
        "date": date,
        "id": id,
        "ac rate": ac_rate,
        "title": title,
        "title cn": title_cn,
    });

    println!("{:#?}", output);

    Ok(())
}

/*
{
    "query": "\n    query problemsetQuestionList( $limit: Int, $skip: Int) {\n  problemsetQuestionList(\n  limit: $limit\n    skip: $skip\n  ) {\n    hasMore\n    total\n    questions {\n      acRate\n      difficulty\n  title\n      titleCn\n      titleSlug\n   }\n  }\n}\n    ",
    "variables": {
        "skip": 0,
        "limit": 100
    },
    "operationName": "problemsetQuestionList"
}
*/
// TODO refactor
pub async fn query_all(path: PathBuf) -> Result<i32> {
    let mut index = 0;

    loop {
        let limit = 100;
        let mut skip = index * limit;

        let query_body = json!({
            "query": "\n query problemsetQuestionList($limit: Int, $skip: Int) {\n problemsetQuestionList(\n limit: $limit\n skip: $skip\n) {\n hasMore\n total\n questions {\n acRate\n difficulty\n title\n titleCn\n titleSlug\n}\n}\n}\n",
            "variables": {
                "skip": skip,
                "limit": limit,
            },
            "operationName": "problemsetQuestionList"
        });

        let mut headers = HeaderMap::new();
        headers.insert(
            "x-csrftoken",
            HeaderValue::from_static(
                "ICuvUTQIutzPLVKDIhUTHk3aGITDbgfsKDMx600p5VeqnLrcB9QS0bPB3ANX0ulS",
            ),
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
            .post("https://leetcode.cn/graphql/")
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
