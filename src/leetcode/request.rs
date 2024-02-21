use super::Url;
use crate::{config, db};
use anyhow::Result;
use log::debug;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::json;

pub struct Request {
    client: Client,
}

impl Default for Request {
    fn default() -> Self {
        if let Ok(client) = Client::builder().build() {
            Request { client }
        } else {
            unreachable!()
        }
    }
}

impl Request {
    pub async fn query_testcase_with_id(&self, id: i32) -> Result<String> {
        let db = db::Sqlite3::global();
        let title_slug = db.query_with_id(id).unwrap().slug().unwrap();

        let query_body = json!({
            "query": vec![
                "query consolePanelConfig($titleSlug: String!) {",
                "   question(titleSlug: $titleSlug) {",
                "       questionId",
                "       questionTitle",
                "       jsonExampleTestcases",
                "       exampleTestcases",
                "       metaData",
                "       sampleTestCase",
                "   }",
                "}",
            ].join("\n"),
            "variables": {
                "titleSlug": title_slug,
            },
            "operationName": "consolePanelConfig"
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
            .post(Url::global().graphql())
            .json(&query_body)
            .send()
            .await?
            .text()
            .await?;

        let resp_json: serde_json::Value = serde_json::from_str(resp.as_str()).unwrap();
        let testcases = &resp_json["data"]["question"]["exampleTestcases"];

        debug!("{:#?}", resp_json);

        Ok(testcases.to_string())
    }
}

mod test {
    use tokio::runtime::Builder;

    use super::Request;

    #[test]
    fn query_testcase_with_id() {
        if let Ok(testcase) = Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("build tokio runtime failed")
            .block_on(Request::default().query_testcase_with_id(198))
        {
            println!("{:#?}", testcase.replace("\"", ""));
        } else {
            println!("access testdata failed");
        }
    }
}
