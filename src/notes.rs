use anyhow::{anyhow, Result};
use chrono::Utc;
use std::{env, fs, process::Command};
use walkdir::WalkDir;

fn get_base_path() -> Result<String> {
    let home = env::var("HOME")?;
    Ok(format!("{}/.rnote/", home))
}

fn get_path(category: &str) -> Result<String> {
    let base = get_base_path()?;
    let date = Utc::now().format("%Y-%m-%d");
    match category.is_empty() {
        true => Ok(format!("{}{}", base, date)),
        false => Ok(format!("{}{}", base, category)),
    }
}

pub fn create_dir(category: &str) -> Result<()> {
    let path = get_base_path()?;
    let date = Utc::now().format("%Y-%m-%d");
    match category.is_empty() {
        true => fs::create_dir_all(format!("{}{}", path, date))?,
        false => fs::create_dir_all(format!("{}{}", path, category))?,
    }
    Ok(())
}

pub fn create_note(header: &str, category: &str) -> Result<()> {
    let editor = env::var("EDITOR").unwrap_or("/bin/vi".to_owned());
    let file = format!("{}{}.md", get_path(category)?, header);
    is_duplicate(header, category)?;
    Command::new(editor).arg(&file).status()?;
    Ok(())
}

fn is_duplicate(header: &str, category: &str) -> Result<()> {
    let file = format!("{}{}", get_path(category)?, header);
    let path = format!("{}", get_path(category)?);
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let p: &str = match entry.path().to_str() {
            Some(s) => s,
            None => "",
        };
        if p == file {
            return Err(anyhow!(
                "Duplicate in the same category/date. Choose another name."
            ));
        }
    }
    Ok(())
}
