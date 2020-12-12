# rnote
[![license](https://img.shields.io/badge/licence-GPLv3-blue)](https://framagit.org/fiplox/rpass/-/blob/main/LICENSE)

A minimal note taking cli tool. 

## Description

**rnote** creates `Markdown` text files (with permissions set to 600) in a date-named or category-named directories (with permissions set to 700) with a name of a header of the note. 

Example:

```
.rnote/
├── 2020-03-20
│   └── Lockdown.md
├── 2020-10-20
│   └── New Lockdown.md
└── shop
    └── to buy.md
```

## Basic usage
`rnote` gives certain subcommands for managing your notes.
```
USAGE:
    rnote [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    edit      Edit a note.
    help      Prints this message or the help of the given subcommand(s)
    list      List all notes or notes from a category.
    new       Create new note
    panic     Delete all notes.
    remove    Remove a note.
    search    Search a note.
    show      Show note(s) in TextView.
```
You can get help page to each subcommand individually by typing `rnote <subcommand> -h` to see all possible options.

## Usage
This section explains the thorough use of each procedure.

* [Create a note](#create-a-note) 
* [Edit a note](#edit-a-note)
* [List notes](#list-notes)
* [Remove notes](#remove-notes)
* [Search notes](#search-notes)
* [Show notes](#search-notes)

### Create a note

### Edit a note

### List notes

### Remove notes

### Search notes

### Show notes

## TODO

- [x] Create a note
- [x] Delete a note
- [x] Modify a note
- [x] Show all notes in a scrollable TextView in a raw terminal with [termimad](https://crates.io/crates/termimad)
- [x] Show all notes from `DATE` or `Category`
- [x] Search a note by header
- [x] Search a note by word (kinda grep)
- [x] List all notes
- [x] List all notes from a `Category`
- [x] Delete all notes (`panic`)
- [x] Delete all notes created at a certain date
