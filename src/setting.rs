use std::{cell::OnceCell, env::home_dir, fs::try_exists};

use anyhow::Result;
use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

use crate::util;

#[derive(Debug, Deserialize)]
struct Project {
    path: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    project: Project,
}

static mut SETTING: OnceCell<Settings> = OnceCell::new();

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let path = home_dir()
            .unwrap()
            .join(".config")
            .join("lctool")
            .join("lc.toml");

        if !try_exists(path).unwrap() {
            util::default_config().unwrap();
        }

        let s = Config::builder()
            .add_source(File::with_name(
                home_dir()
                    .unwrap()
                    .join(".config")
                    .join("lctool")
                    .join("lc.toml")
                    .to_str()
                    .unwrap(),
            ))
            .build()?;

        s.try_deserialize()
    }

    pub fn global() -> &'static Self {
        unsafe { SETTING.get_or_init(|| Settings::new().unwrap()) }
    }

    pub fn path(&self) -> Result<String> {
        Ok(self.project.path.clone())
    }
}
