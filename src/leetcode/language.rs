use std::str::FromStr;

#[derive(Debug)]
pub enum Language {
    Cpp,
    Rust,
    Java,
    Go,
    Python,
    JavaScript,
    TypeScript,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Cpp => String::from("cpp"),
            Language::Rust => String::from("rs"),
            Language::Java => String::from("java"),
            Language::Go => String::from("go"),
            Language::Python => String::from("py"),
            Language::JavaScript => String::from("js"),
            Language::TypeScript => String::from("ts"),
        }
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cpp" => Ok(Language::Cpp),
            "rust" => Ok(Language::Rust),
            "java" => Ok(Language::Java),
            "go" => Ok(Language::Go),
            "python" => Ok(Language::Python),
            "js" => Ok(Language::JavaScript),
            "ts" => Ok(Language::TypeScript),
            _ => Err(()),
        }
    }
}
