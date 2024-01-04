mod cookies;
mod storage;

use anyhow::Result;
use config::{Config as ConfigPaser, ConfigError, File};
use serde_derive::Deserialize;
use std::cell::OnceCell;

use self::{cookies::Cookies, storage::Storage};

static mut CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub cookies: Cookies,
    pub storage: Storage,
}

impl Config {
    fn new() -> Result<Config, ConfigError> {
        let path = dir::home_dir()
            .unwrap()
            .join(".config")
            .join("leetcode")
            .join("leetcode.toml")
            .to_string_lossy()
            .to_string();

        let config = ConfigPaser::builder()
            .add_source(File::with_name(&path))
            .build()?;

        config.try_deserialize()
    }

    pub fn global() -> &'static Self {
        unsafe { CONFIG.get_or_init(|| Config::new().unwrap()) }
    }
}

mod test {
    use super::Config;

    #[test]
    fn access_config() {
        let config = Config::global();

        println!("root = {}", config.storage.root().unwrap());
        println!("cache = {}", config.storage.cache().unwrap());
        println!("proj = {}", config.storage.project().unwrap());

        println!("csrf = {}", config.cookies.csrf().unwrap());
        println!("session = {}", config.cookies.session().unwrap());
    }
}
