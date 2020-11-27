use anyhow::Result;
use chrono::Utc;
use std::{env, fs, process::Command};

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
    let path = format!("{}{}", get_path(category)?, header);
    // TODO: check if duplicate
    Command::new(editor).arg(&path).status()?;
    Ok(())
}
