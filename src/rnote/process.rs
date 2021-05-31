use crate::rnote::notes;
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use dialoguer::{theme::ColorfulTheme, Input};

/// Process argument `new`.
pub fn new(matches: &ArgMatches) -> Result<()> {
    let name = match matches.value_of("name") {
        Some(s) => s.to_owned(),
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name of your note")
            .interact_text()?,
    };
    let category = match matches.value_of("category") {
        Some(s) => s.to_owned(),
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Category for your note")
            .default("".to_string())
            .interact_text()?,
    };

    notes::create(&name, &category)?;
    Ok(())
}

/// Process argument `remove`.
pub fn remove(matches: &ArgMatches) -> Result<()> {
    if matches.is_present("date") {
        let date: String = match matches.value_of("name") {
            Some(s) => s.to_string(),
            None => Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Date")
                .interact_text()?,
        };
        return notes::remove_by_date(&date);
    }
    if matches.is_present("category") {
        let category: String = match matches.value_of("name") {
            Some(s) => s.to_string(),
            None => Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Category")
                .interact_text()?,
        };
        return notes::remove_category(&category);
    }
    let name: String = match matches.value_of("name") {
        Some(s) => s.to_string(),
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name")
            .interact_text()?,
    };
    return notes::remove_note(&name);
}

/// Process argument `remove`.
pub fn edit(matches: &ArgMatches) -> Result<()> {
    let name = match matches.value_of("name") {
        Some(s) => s.to_owned(),
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name of your note")
            .interact_text()?,
    };

    notes::modify(&name)?;
    Ok(())
}

/// Process argument `list`.
pub fn list(matches: &ArgMatches) -> Result<()> {
    match matches.is_present("category") {
        true => {
            let name: String = match matches.value_of("name") {
                Some(s) => s.to_string(),
                None => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Category:")
                    .interact_text()?,
            };
            notes::list_category(&name)?;
        }
        false => notes::list_all_notes()?,
    }
    Ok(())
}

/// Process argument `search`.
pub fn search(matches: &ArgMatches) -> Result<()> {
    if matches.is_present("word") {
        let word: String = match matches.value_of("name") {
            Some(s) => s.to_string(),
            None => Input::with_theme(&ColorfulTheme::default())
                .with_prompt("String to search")
                .interact_text()?,
        };
        return notes::search_by_word(&word);
    }

    match matches.value_of("name") {
        Some(s) => {
            let p = notes::get_note_path_interractive(s)?;
            match p {
                Some(s) => {
                    let editor = std::env::var("EDITOR")?;
                    std::process::Command::new(editor).arg(s).status()?;
                }
                None => return Err(anyhow!("Nothing found.")),
            }
        }
        None => return Err(anyhow!("Nothing entered for search.")),
    }
    Ok(())
}

/// Process argument `show`.
pub fn show(matches: &ArgMatches) -> Result<()> {
    if matches.is_present("all") {
        return notes::show_all();
    }
    if matches.is_present("category") {
        let category: String = match matches.value_of("name") {
            Some(s) => s.to_string(),
            None => Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Category:")
                .interact_text()?,
        };
        return notes::show_category(&category);
    }

    match matches.value_of("name") {
        Some(s) => return notes::show(s),
        None => {
            let s: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("String to search")
                .interact_text()?;
            return notes::show(&s);
        }
    }
}

/// Process argument `panic`.
pub fn panic() -> Result<()> {
    let base = notes::get_base_path()?;
    std::fs::remove_dir_all(base)?;
    Ok(())
}
