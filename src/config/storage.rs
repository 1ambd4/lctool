use std::{fs, path::PathBuf};

use anyhow::{Ok, Result};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Storage {
    cache: String,
    project: String,
    root: String,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            cache: "leetcode.sqlite".into(),
            project: "code".into(),
            root: "~/.config/leetcode/".into(),
        }
    }
}

impl Storage {
    pub fn root(&self) -> Result<String> {
        let home = dir::home_dir().unwrap().to_string_lossy().to_string();
        let path = self.root.replace("~", &home);
        Ok(path)
    }

    pub fn cache(&self) -> Result<String> {
        let root = PathBuf::from(self.root()?);
        if !root.exists() {
            fs::DirBuilder::new().recursive(true).create(&root)?;
        }
        Ok(root.join(&self.cache).to_string_lossy().to_string())
    }

    pub fn project(&self) -> Result<String> {
        let home = dir::home_dir().unwrap().to_string_lossy().to_string();
        let proj_path = PathBuf::from(&self.project);
        let path = proj_path.to_string_lossy().to_string().replace("~", &home);

        Ok(path)
    }
}
