use std::{
    env::home_dir,
    fs::{create_dir_all, remove_dir_all, remove_file, try_exists, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Result;

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

pub fn default_config() -> Result<()> {
    println!("Please input your project path:");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");
    buf = buf.trim().to_string();

    let mut project_path;

    if buf.is_empty() {
        project_path = home_dir().unwrap().join("github").join("leetcode");
    } else {
        project_path = PathBuf::from(buf.to_string());
    }

    let config_path = home_dir().unwrap().join(".config").join("lctool");

    if !try_exists(&config_path).unwrap() {
        create_dir_all(&config_path).unwrap();
    }

    let config_file = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(config_path.join("lc.toml"))
        .unwrap();

    let mut writer = BufWriter::new(config_file);
    writer.write_all("[project]\n".as_bytes()).unwrap();
    writer
        .write_all(format!("path = \"{}\"\n", project_path.to_str().unwrap()).as_bytes())
        .unwrap();

    Ok(())
}
