use crate::rnote::notes;
use anyhow::{anyhow, Result};
use clap::ArgMatches;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

pub fn new(matches: &ArgMatches) -> Result<()> {
    let header = match matches.value_of("header") {
        Some(s) => s.to_owned(),
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name of your note")
            .interact_text()?,
    };
    let category = matches.value_of("category").unwrap_or("");

    notes::create(&header, category)?;
    Ok(())
}

pub fn remove(matches: &ArgMatches) -> Result<()> {
    let header = match matches.value_of("header") {
        Some(s) => s.to_owned(),
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name of your note")
            .interact_text()?,
    };
    notes::remove(&header)?;
    Ok(())
}

pub fn edit(matches: &ArgMatches) -> Result<()> {
    let header = match matches.value_of("header") {
        Some(s) => s.to_owned(),
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Name of your note")
            .interact_text()?,
    };

    notes::modify(&header)?;
    Ok(())
}

pub fn list(matches: &ArgMatches) -> Result<()> {
    unimplemented!("list all notes, one note or category {:?}", matches);
}

pub fn search(matches: &ArgMatches) -> Result<()> {
    match matches.value_of("header") {
        Some(s) => {
            let p = notes::find_path(s)?;
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to open it?")
                .default(true)
                .interact()?
            {
                let editor = std::env::var("EDITOR")?;
                std::process::Command::new(editor).arg(&p).status()?;
            } else {
                println!("{}", p);
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

pub fn show(matches: &ArgMatches) -> Result<()> {
    match matches.value_of("header") {
        Some(s) => unimplemented!("{}", s),
        None => match matches.is_present("all") {
            true => notes::show_all()?,
            false => return Err(anyhow!("No option is given. Abort.")),
        },
    }
    Ok(())
}
