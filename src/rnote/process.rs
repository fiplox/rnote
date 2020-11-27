use crate::rnote::notes;
use anyhow::Result;
use clap::ArgMatches;
use notes::modify;
use text_io::read;

pub fn new(matches: &ArgMatches) -> Result<()> {
    let header = match matches.value_of("header") {
        Some(s) => s.to_owned(),
        None => {
            print!("Enter the name of your note: ");
            read!()
        }
    };
    let category = matches.value_of("category").unwrap_or("");

    notes::create(&header, category)?;
    Ok(())
}
pub fn remove(matches: &ArgMatches) -> Result<()> {
    let header = match matches.value_of("header") {
        Some(s) => s.to_owned(),
        None => {
            print!("Enter the name of your note: ");
            read!()
        }
    };
    notes::remove(&header)?;
    Ok(())
}
pub fn edit(matches: &ArgMatches) -> Result<()> {
    let header = match matches.value_of("header") {
        Some(s) => s.to_owned(),
        None => {
            print!("Enter the name of your note: ");
            read!()
        }
    };

    notes::modify(&header)?;
    Ok(())
}
pub fn list(matches: &ArgMatches) -> Result<()> {
    unimplemented!("list all notes, one note or category");
    Ok(())
}
pub fn search(matches: &ArgMatches) -> Result<()> {
    unimplemented!("Search a note by header or by word.");
    Ok(())
}
