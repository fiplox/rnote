use anyhow::{anyhow, Result};
use chrono::Utc;
use std::{env, fs, process::Command};
use text_io::read;
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

fn find_path(header: &str) -> Result<String> {
    let mut paths: Vec<String> = Vec::new();
    let base = get_base_path()?;
    for entry in WalkDir::new(base) {
        let entry = entry?;
        let p: &str = match entry.path().to_str() {
            Some(s) => s,
            None => "",
        };
        let name: &str = match entry.file_name().to_str() {
            Some(s) => s,
            None => "",
        };
        if name == header {
            paths.push(String::from(p));
        }
    }
    if paths.is_empty() {
        Err(anyhow!("Note not found."))
    } else {
        if paths.len() == 1 {
            Ok(paths.remove(0))
        } else {
            let mut n: usize;
            loop {
                let mut i = 1;
                println!("Choose one: \n");
                for path in &paths {
                    println!("{}\t {}", i, path);
                    i += 1;
                }
                n = read!();
                if n >= 1 && n <= paths.len() {
                    break;
                }
            }
            Ok(paths.remove(n))
        }
    }
}

pub fn delete_note(header: &str) -> Result<()> {
    let path = find_path(header)?;
    println!("Are you sure you want to delete {} [Y/n]", header);
    let response: String = read!();
    if response == "y" || response == "Y" || response == "yes" || response == "Yes" {
        println!("Deleting...");
        fs::remove_file(path)?;
        println!("Successfully deleted.");
        Ok(())
    } else {
        Err(anyhow!("Abort."))
    }
}
