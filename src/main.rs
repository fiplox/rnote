use anyhow::Result;
use rnote::{app, process};

mod rnote;

fn main() -> Result<()> {
    let mut app = app::make_app();

    match rnote::app::make_app().get_matches().subcommand() {
        ("new", Some(m)) => process::new(m)?,
        ("remove", Some(m)) => process::remove(m)?,
        ("edit", Some(m)) => process::edit(m)?,
        ("list", Some(m)) => process::list(m)?,
        ("search", Some(m)) => process::search(m)?,
        _ => app.print_long_help()?,
    };

    Ok(())
}
