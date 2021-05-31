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
    match matches.value_of("name") {
        Some(s) => notes::remove_interractive(s)?,
        None => match matches.is_present("date") {
            true => {
                let date: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Date")
                    .interact_text()?;
                notes::wipe_date(&date)?;
                return Ok(());
            }
            false => {
                let name: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Name of your note")
                    .interact_text()?;
                notes::remove_interractive(&name)?;
            }
        },
    }
    Ok(())
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
            let s: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Category:")
                .interact_text()?;
            notes::list_category(&s)?;
        }
        false => notes::list_all_notes()?,
    }
    Ok(())
}

/// Process argument `search`.
pub fn search(matches: &ArgMatches) -> Result<()> {
    match matches.value_of("name") {
        Some(s) => {
            let p = notes::get_note_path_interractive(s)?;
            match p {
                Some(s) => {
                    let editor = std::env::var("EDITOR")?;
                    std::process::Command::new(editor).arg(s).status()?;
                }
                None => (),
            }
        }
        None => match matches.is_present("word") {
            true => {
                let s: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("String to search")
                    .interact_text()?;
                notes::search_by_word(&s)?;
            }
            false => return Err(anyhow!("Nothing entered for search.")),
        },
    }
    Ok(())
}

/// Process argument `show`.
pub fn show(matches: &ArgMatches) -> Result<()> {
    match matches.value_of("name") {
        Some(s) => notes::show(s)?,
        None => match matches.is_present("all") {
            true => notes::show_all()?,
            false => match matches.is_present("category") {
                true => {
                    let category: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Name of category:")
                        .interact_text()?;
                    notes::show_category(&category)?;
                }
                false => {
                    let s: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("String to search")
                        .interact_text()?;
                    notes::show(&s)?;
                }
            },
        },
    }
    Ok(())
}

/// Process argument `panic`.
pub fn panic() -> Result<()> {
    let base = notes::get_base_path()?;
    std::fs::remove_dir_all(base)?;
    Ok(())
}
