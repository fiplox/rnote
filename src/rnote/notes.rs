use crate::rnote::show;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use dialoguer::{theme::ColorfulTheme, Select};
use std::{env, fs, io::Write, os::unix::fs::PermissionsExt, path::PathBuf, process::Command};
use walkdir::WalkDir;

/// Get the path to the root directory of all notes.
pub fn get_base_path() -> Result<String> {
    let home = env::var("XDG_DATA_HOME")?;
    Ok(format!("{}/rnote/", home))
}

/// Get path to a category/date directory.
fn get_category_path(category: &str) -> Result<String> {
    let base = get_base_path()?;
    let date = Utc::now().format("%Y-%m-%d");
    match category.is_empty() {
        true => Ok(format!("{}{}/", base, date)),
        false => Ok(format!("{}{}/", base, category)),
    }
}

/// Get all note paths.
pub fn get_all_notes() -> Result<Vec<String>> {
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
    if files.is_empty() {
        Err(anyhow!("No notes found."))
    } else {
        Ok(files)
    }
}

/// Get all notes in category.
pub fn get_notes_in_category(category: &str) -> Result<Vec<String>> {
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
        if files.is_empty() {
            Err(anyhow!("Category is empty."))
        } else {
            Ok(files)
        }
    } else {
        Err(anyhow!("Category no found."))
    }
}

/// Create directory for a note.
pub fn create_dir(category: &str) -> Result<()> {
    let base = get_base_path()?;
    let date = Utc::now().format("%Y-%m-%d");
    match category.is_empty() {
        true => {
            fs::create_dir_all(format!("{}{}", base, date))?;
            fs::set_permissions(
                format!("{}{}", base, date),
                fs::Permissions::from_mode(0o700),
            )?;
        }
        false => {
            fs::create_dir_all(format!("{}{}", base, category))?;
            fs::set_permissions(
                format!("{}{}", base, category),
                fs::Permissions::from_mode(0o700),
            )?;
        }
    }
    fs::set_permissions(base, fs::Permissions::from_mode(0o700))?;
    Ok(())
}

/// Find a path to desired note.
pub fn get_note_path(name: &str) -> Result<Vec<String>> {
    let mut paths: Vec<String> = Vec::new();
    let base = get_base_path()?;
    let name = format!("{}.md", name);
    for entry in WalkDir::new(base) {
        let entry = entry?;
        let p: String = match entry.path().to_str() {
            Some(s) => s.to_owned(),
            None => "".to_owned(),
        };
        let file_name: &str = match entry.file_name().to_str() {
            Some(s) => s,
            None => "",
        };
        if file_name == name {
            paths.push(p);
        }
    }
    if paths.is_empty() {
        Err(anyhow!("Note not found."))
    } else {
        Ok(paths)
    }
}

/// Find all notes that contain a given string.
pub fn get_files_by_word(word: &str) -> Result<Vec<String>> {
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
    Ok(paths)
}

/// Create a new note.
pub fn create(name: &str, category: &str) -> Result<()> {
    let editor = env::var("EDITOR")?;
    let file = format!("{}{}.md", get_category_path(category)?, name);
    create_dir(category)?;
    is_duplicate(name, category)?;
    let mut f = fs::File::create(&file)?;
    let username = env::var("USER").unwrap_or("".to_owned());
    let date = Utc::now().format("%d-%m-%Y");
    let note_name = format!(
        r#"---
title: {}
author: {}
date: {}
---"#,
        name, username, date
    );
    f.set_permissions(fs::Permissions::from_mode(0o600))?;
    f.write(format!("{}\n", note_name).as_bytes())?;
    Command::new(editor).arg(&file).status()?;
    Ok(())
}

/// Check if potentially new note name already exists.
fn is_duplicate(name: &str, category: &str) -> Result<()> {
    let file = format!("{}{}.md", get_category_path(category)?, name);
    let path = format!("{}", get_category_path(category)?);
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

/// Find a path to desired note and prompt to choose one to open.
pub fn get_note_path_interractive(name: &str) -> Result<Option<String>> {
    let mut paths: Vec<String> = get_note_path(name)?;
    let mut p: Vec<String> = paths.clone();
    let r = p[0].find("rnote").unwrap_or(0);
    p = p.into_iter().map(|mut s| s.drain(r..).collect()).collect();
    if paths.len() == 1 {
        Ok(Some(paths.remove(0)))
    } else {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Optionally choose a note")
            .default(0)
            .items(&p)
            .interact_opt()?;
        match selection {
            Some(s) => Ok(Some(paths.remove(s))),
            None => Ok(None),
        }
    }
}

/// Delete a note.
pub fn remove_note(name: &str) -> Result<()> {
    let mut paths = get_note_path(name)?;
    if paths.len() == 1 {
        println!("Deleting...");
        fs::remove_file(paths.remove(0))?;
        remove_empty_dirs()?;
        println!("Successfully deleted.");
        Ok(())
    } else {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a note to delete")
            .items(&paths)
            .interact_opt()?;
        match selection {
            Some(s) => {
                fs::remove_file(paths.remove(s))?;
                remove_empty_dirs()?;
                println!("Successfully deleted.");
                Ok(())
            }
            None => {
                println!("Canceling...");
                Ok(())
            }
        }
    }
}

pub fn remove_category(category: &str) -> Result<()> {
    let path = get_category_path(category)?;
    println!("Deleting...");
    fs::remove_dir_all(path)?;
    println!("Successfully deleted.");
    Ok(())
}

/// Modify a note.
pub fn modify(name: &str) -> Result<()> {
    let editor = env::var("EDITOR")?;
    let file = get_note_path_interractive(name)?;
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

/// Prompt user to open one of found notes by word.
pub fn search_by_word(word: &str) -> Result<()> {
    let mut paths: Vec<String> = get_files_by_word(word)?;
    let mut p: Vec<String> = paths.clone();
    let r = p[0].find("rnote").unwrap_or(0);
    p = p.into_iter().map(|mut s| s.drain(r..).collect()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally choose a note")
        .default(0)
        .items(&p)
        .interact_opt()?;
    if let Some(selection) = selection {
        let editor = std::env::var("EDITOR")?;
        std::process::Command::new(editor)
            .arg(paths.remove(selection))
            .status()?;
    }

    Ok(())
}

/// Show all notes.
pub fn show_all() -> Result<()> {
    let base: String = get_base_path()?;
    let mut files: Vec<String> = Vec::new();
    for (_, file) in WalkDir::new(base)
        .into_iter()
        .filter_map(|file| file.ok())
        .enumerate()
    {
        if file.metadata()?.is_file() {
            files.push(fs::read_to_string(file.path())?);
        }
    }
    if files.is_empty() {
        return Err(anyhow!("No notes found."));
    }
    let skin = show::make_skin();
    let md = &files.join("---\n");
    show::run_app(skin, md)?;
    Ok(())
}

/// Show one note.
pub fn show(name: &str) -> Result<()> {
    let path = get_note_path_interractive(name)?;
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

/// Show all notes in the given category.
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
        if files.is_empty() {
            return Err(anyhow!("No notes found."));
        }
        let skin = show::make_skin();
        let md = &files.join("---\n");
        show::run_app(skin, md)?;
        Ok(())
    } else {
        Err(anyhow!("Category does not exist."))
    }
}

/// List all notes and prompt to open one.
pub fn list_all_notes() -> Result<()> {
    let mut files: Vec<String> = get_all_notes()?;
    let mut p: Vec<String> = files.clone();
    let r = p[0].find("rnote").unwrap_or(0);
    p = p.into_iter().map(|mut s| s.drain(r..).collect()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally choose a note")
        .default(0)
        .items(&p)
        .interact_opt()?;
    if let Some(selection) = selection {
        let editor = std::env::var("EDITOR")?;
        std::process::Command::new(editor)
            .arg(files.remove(selection))
            .status()?;
    }
    Ok(())
}

/// List all notes in the given category and optionally open one.
pub fn list_category(category: &str) -> Result<()> {
    let mut files: Vec<String> = get_notes_in_category(category)?;
    let mut p: Vec<String> = files.clone();
    let r = p[0].find("rnote").unwrap_or(0);
    p = p.into_iter().map(|mut s| s.drain(r..).collect()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally choose a note")
        .default(0)
        .items(&p)
        .interact_opt()?;
    if let Some(selection) = selection {
        let editor = std::env::var("EDITOR")?;
        std::process::Command::new(editor)
            .arg(files.remove(selection))
            .status()?;
    }
    Ok(())
}

/// Remove all notes created at the given date in format `YYYY-MM-dd`.
pub fn remove_by_date(date: &str) -> Result<()> {
    let base = get_base_path()?;
    for (_, file) in WalkDir::new(base)
        .into_iter()
        .filter_map(|file| file.ok())
        .enumerate()
    {
        if file.metadata()?.is_file() {
            let time: DateTime<Utc> = file.metadata()?.created()?.into();
            if time.format("%Y-%m-%d").to_string() == date {
                // TODO: add verbose flag and prompt to delete each file.
                fs::remove_file(file.path())?;
            }
        }
    }
    remove_empty_dirs()?;

    Ok(())
}

/// Remove empty directories.
fn remove_empty_dirs() -> Result<()> {
    let base = get_base_path()?;
    for (_, file) in WalkDir::new(base)
        .into_iter()
        .filter_map(|file| file.ok())
        .enumerate()
    {
        let is_empty = PathBuf::from(file.path())
            .read_dir()
            .map(|mut i| i.next().is_none())
            .unwrap_or(false);
        if is_empty {
            fs::remove_dir(file.path())?;
        }
    }

    Ok(())
}

// Make sure to remove rnote directory before tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_base_path_test() {
        assert!(get_base_path().is_ok());
    }

    #[test]
    fn find_by_word_test() {
        assert!(create("test", "test_word").is_ok());
        assert!(get_files_by_word("test").is_ok());
    }

    #[test]
    fn get_note_path_test() {
        assert!(create("test", "test_path").is_ok());
        assert!(get_note_path("test").is_ok());
    }

    #[test]
    fn get_category_path_create_dir_test() {
        assert!(create_dir("test_dir").is_ok());
        assert!(get_category_path("test").is_ok());
    }

    #[test]
    fn create_remove_test() {
        assert!(create("test1", "test1").is_ok());
        let data_home = std::env::var("XDG_DATA_HOME").unwrap_or("".to_owned());
        assert!(remove_note(&format!("{}/rnote/test1/test1.md", data_home)).is_ok());
    }

    #[test]
    fn remove_empty_dirs_test() {
        assert!(create_dir("test_empty").is_ok());
        assert!(remove_empty_dirs().is_ok());
    }

    #[test]
    #[ignore]
    fn remove_by_date_test() {
        assert!(remove_by_date("1999-10-10").is_ok());
    }

    #[test]
    fn get_notes_in_category_test() {
        assert!(create("test", "test_c").is_ok());
        assert!(get_notes_in_category("test_c").is_ok());
    }
}
