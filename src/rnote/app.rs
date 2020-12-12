pub use clap::{App, AppSettings, Arg, SubCommand};

/// Initialize all possible arguments.
pub fn make_app() -> App<'static, 'static> {
    App::new("rnote")
        .version("0.0.1")
        .author("Volodymyr Patuta <vpatuta AT protonmail DOT com>")
        .about("Minimal note talking cli tool.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("Create new note")
                .arg(
                    Arg::with_name("header")
                        .index(1)
                        .help("Give name to the note."),
                )
                .arg(
                    Arg::with_name("category")
                        .help("Create note in category.")
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .alias("r")
                .about("Remove a note.")
                .arg(Arg::with_name("header").help("Name of the note."))
                .arg(
                    Arg::with_name("date")
                        .help("Delete all notes created at given date.")
                        .short("d")
                        .long("date"),
                ),
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
                .about("List all notes or notes from a category")
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
                .arg(
                    Arg::with_name("word")
                        .help("Search by word.")
                        .short("w")
                        .long("word"),
                )
                .arg(Arg::with_name("header").help("Name of the note.")),
        )
        .subcommand(
            SubCommand::with_name("show")
                .arg(
                    Arg::with_name("all")
                        .help("Show all notes.")
                        .short("a")
                        .long("all"),
                )
                .arg(
                    Arg::with_name("category")
                        .help("Show all notes from a category/date")
                        .short("c")
                        .long("category"),
                )
                .arg(Arg::with_name("header").help("Name of the note.")),
        )
        .subcommand(SubCommand::with_name("panic").help("Delete all notes."))
}
