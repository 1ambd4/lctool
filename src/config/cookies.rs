use anyhow::Result;
use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Cookies {
    csrf: String,
    session: String,
}

impl ToString for Cookies {
    fn to_string(&self) -> String {
        format!("csrftoken={};LEETCODE_SESSION={};", self.csrf, self.session)
    }
}

impl Cookies {
    pub fn csrf(&self) -> Result<String> {
        Ok(self.csrf.clone())
    }

    pub fn session(&self) -> Result<String> {
        Ok(self.session.clone())
    }
}
