use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use rnote::{app, process};

mod rnote;

fn check() -> Result<()> {
    let editor = std::env::var("EDITOR").unwrap_or("".to_owned());
    if editor.is_empty() {
        let editor: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your text editor")
            .interact_text()?;
        std::env::set_var("EDITOR", editor);
    }

    Ok(())
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
        _ => app.print_long_help()?,
    };

    Ok(())
}
