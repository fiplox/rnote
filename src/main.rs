use anyhow::{anyhow, Result};
use rnote::{app, process};

mod rnote;

/// Check if variable `EDITOR` and `XDG_DATA_HOME` are set.
fn check() -> Result<()> {
    let editor = std::env::var("EDITOR").unwrap_or("".to_owned());
    let data_home = std::env::var("XDG_DATA_HOME").unwrap_or("".to_owned());
    if editor.is_empty() || data_home.is_empty() {
        Err(anyhow!(
            "Please make sure variables EDITOR and XDG_DATA_HOME are set."
        ))
    } else {
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut app = app::make_app();
    check()?;

    match rnote::app::make_app().get_matches().subcommand() {
        ("new", Some(m)) => process::new(m)?,
        ("remove", Some(m)) => process::remove(m)?,
        ("edit", Some(m)) => process::edit(m)?,
        ("list", Some(m)) => process::list(m)?,
        ("show", Some(m)) => process::show(m)?,
        ("search", Some(m)) => process::search(m)?,
        ("panic", _) => process::panic()?,
        _ => app.print_long_help()?,
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn check_test() {
        assert!(check().is_ok());
    }
}
