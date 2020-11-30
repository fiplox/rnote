use crate::rnote::show;
use anyhow::{anyhow, Result};
use chrono::Utc;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::{env, fs, io::Write, os::unix::fs::PermissionsExt, process::Command};
use walkdir::WalkDir;

/// Get the path to the root directory of all notes.
fn get_base_path() -> Result<String> {
    let home = env::var("HOME")?;
    Ok(format!("{}/.rnote/", home))
}

/// Get path to a category/date directory.
fn get_path(category: &str) -> Result<String> {
    let base = get_base_path()?;
    let date = Utc::now().format("%Y-%m-%d");
    match category.is_empty() {
        true => Ok(format!("{}{}/", base, date)),
        false => Ok(format!("{}{}/", base, category)),
    }
}

/// Create directory for a note.
pub fn create_dir(category: &str) -> Result<()> {
    let path = get_base_path()?;
    let date = Utc::now().format("%Y-%m-%d");
    match category.is_empty() {
        true => fs::create_dir_all(format!("{}{}", path, date))?,
        false => fs::create_dir_all(format!("{}{}", path, category))?,
    }
    Ok(())
}

/// Create a new note.
pub fn create(header: &str, category: &str) -> Result<()> {
    let editor = env::var("EDITOR")?;
    let file = format!("{}{}.md", get_path(category)?, header);
    create_dir(category)?;
    is_duplicate(header, category)?;
    let mut f = fs::File::create(&file)?;
    f.set_permissions(fs::Permissions::from_mode(0o600))?;
    f.write(format!("# {}\n", header).as_bytes())?;
    Command::new(editor).arg(&file).status()?;
    Ok(())
}

/// Checks if potentially new note name already exists.
fn is_duplicate(header: &str, category: &str) -> Result<()> {
    let file = format!("{}{}.md", get_path(category)?, header);
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

/// Finds a path to desired note.
pub fn find_path(header: &str) -> Result<Option<String>> {
    let mut paths: Vec<String> = Vec::new();
    let base = get_base_path()?;
    let header = format!("{}.md", header);
    for entry in WalkDir::new(base) {
        let entry = entry?;
        let p: String = match entry.path().to_str() {
            Some(s) => s.to_owned(),
            None => "".to_owned(),
        };
        let name: &str = match entry.file_name().to_str() {
            Some(s) => s,
            None => "",
        };
        if name == header {
            paths.push(p);
        }
    }
    if paths.is_empty() {
        Err(anyhow!("Note not found."))
    } else {
        if paths.len() == 1 {
            Ok(Some(paths.remove(0)))
        } else {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Optionally choose a note")
                .default(0)
                .items(&paths)
                .interact_opt()?;
            match selection {
                Some(s) => Ok(Some(paths.remove(s))),
                None => Ok(None),
            }
        }
    }
}

/// Deletes a note.
pub fn remove(header: &str) -> Result<()> {
    let path = find_path(header)?;
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Do you want to delete {}?", header))
        .interact()?
        && path.is_some()
    {
        println!("Deleting...");
        fs::remove_file(path.unwrap())?;
        println!("Successfully deleted.");
        Ok(())
    } else {
        Err(anyhow!("Abort."))
    }
}

/// Modify a note.
pub fn modify(header: &str) -> Result<()> {
    let editor = env::var("EDITOR")?;
    let file = find_path(header)?;
    match file {
        Some(f) => {
            Command::new(editor).arg(f).status()?;
            println!("Edited successfully!");
            Ok(())
        }
        None => {
            println!("Abort.");
            Ok(())
        }
    }
}

pub fn search_by_word(word: &str) -> Result<()> {
    extern crate fstream;
    let path = get_base_path()?;
    let mut paths: Vec<String> = Vec::new();
    for (_, file) in WalkDir::new(path)
        .into_iter()
        .filter_map(|file| file.ok())
        .enumerate()
    {
        if file.metadata()?.is_file() {
            match fstream::contains(file.path(), word) {
                Some(b) => {
                    if b {
                        let path = file.path().to_str().unwrap_or("");
                        if !path.is_empty() {
                            paths.push(path.to_owned());
                        }
                    }
                }
                None => continue,
            }
        }
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally choose a note")
        .default(0)
        .items(&paths)
        .interact_opt()?;
    if let Some(selection) = selection {
        let editor = std::env::var("EDITOR")?;
        std::process::Command::new(editor)
            .arg(paths.remove(selection))
            .status()?;
    }

    Ok(())
}

pub fn show_all() -> Result<()> {
    let path = get_base_path()?;
    let mut files: Vec<String> = Vec::new();
    for (_, file) in WalkDir::new(path)
        .into_iter()
        .filter_map(|file| file.ok())
        .enumerate()
    {
        if file.metadata()?.is_file() {
            files.push(fs::read_to_string(file.path())?);
        }
    }
    let skin = show::make_skin();
    let md = &files.join("---\n");
    show::run_app(skin, md)?;
    Ok(())
}

pub fn show(header: &str) -> Result<()> {
    let path = find_path(header)?;
    match path {
        Some(s) => {
            let skin = show::make_skin();
            let content = fs::read_to_string(s)?;
            show::run_app(skin, &content)?;
            Ok(())
        }
        None => Err(anyhow!("Abort.")),
    }
}

pub fn show_category(category: &str) -> Result<()> {
    let base = get_base_path()?;
    let path = format!("{}{}", base, category);
    let mut files: Vec<String> = Vec::new();
    if std::path::Path::new(&path).exists() {
        for (_, file) in WalkDir::new(path)
            .into_iter()
            .filter_map(|file| file.ok())
            .enumerate()
        {
            if file.metadata()?.is_file() {
                files.push(fs::read_to_string(file.path())?);
            }
        }
        let skin = show::make_skin();
        let md = &files.join("---\n");
        show::run_app(skin, md)?;
    }
    Ok(())
}

pub fn list_all() -> Result<()> {
    let path = get_base_path()?;
    let mut files: Vec<String> = Vec::new();
    for (_, file) in WalkDir::new(path)
        .into_iter()
        .filter_map(|file| file.ok())
        .enumerate()
    {
        if file.metadata()?.is_file() {
            let p = file.path().to_str().unwrap_or("");
            if !p.is_empty() {
                files.push(p.to_owned());
            }
        }
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally choose a note")
        .default(0)
        .items(&files)
        .interact_opt()?;
    if let Some(selection) = selection {
        let editor = std::env::var("EDITOR")?;
        std::process::Command::new(editor)
            .arg(files.remove(selection))
            .status()?;
    }
    Ok(())
}

pub fn list_category(category: &str) -> Result<()> {
    let base = get_base_path()?;
    let path = format!("{}{}", base, category);
    let mut files: Vec<String> = Vec::new();
    if std::path::Path::new(&path).exists() {
        for (_, file) in WalkDir::new(path)
            .into_iter()
            .filter_map(|file| file.ok())
            .enumerate()
        {
            if file.metadata()?.is_file() {
                let p = file.path().to_str().unwrap_or("");
                if !p.is_empty() {
                    files.push(p.to_owned());
                }
            }
        }
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Optionally choose a note")
            .default(0)
            .items(&files)
            .interact_opt()?;
        if let Some(selection) = selection {
            let editor = std::env::var("EDITOR")?;
            std::process::Command::new(editor)
                .arg(files.remove(selection))
                .status()?;
        }
    }
    Ok(())
}
