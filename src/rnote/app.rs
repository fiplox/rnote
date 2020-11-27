pub use clap::{App, AppSettings, Arg, SubCommand};

pub fn make_app() -> App<'static, 'static> {
    App::new("rnote")
        .version("0.0.0")
        .author("Volodymyr Patuta <vpatuta AT protonmail DOT com>")
        .about("Minimal note talking cli tool.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("Create new note")
                .arg(
                    Arg::with_name("category")
                        .help("Create note in category.")
                        .short("c")
                        .long("category"),
                )
                .arg(Arg::with_name("header").help("Give name to the file.")),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .alias("r")
                .about("Remove a note.")
                .arg(Arg::with_name("header").help("Name of the note.")),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .alias("e")
                .about("Edit a note.")
                .arg(Arg::with_name("header").help("Name of the note.")),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("l")
                .alias("ls")
                .alias("show")
                .about("Show all notes or one note")
                .arg(Arg::with_name("header").help("Name of the note."))
                .arg(
                    Arg::with_name("category")
                        .help("List all notes from a category.")
                        .short("c")
                        .long("category"),
                ),
        )
        .subcommand(
            SubCommand::with_name("search")
                .alias("s")
                .about("Search a note.")
                .arg(Arg::with_name("header").help("Name of the note."))
                .arg(
                    Arg::with_name("word")
                        .help("Search by word.")
                        .short("w")
                        .long("word"),
                ),
        )
}
