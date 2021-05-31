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
`rnote new` can be simplified by `rnote n`.\
In order to create a note, type:
```
$ rnote new
```
It will prompt you to enter a note name and a note will be saved in date-named category.\
Or
```
$ rnote new <name>
```
Same as previous but without prompt.\
Or
```
$ rnote new <name> <category>
```
It will create a note `<name>` in `<category>`.

### Edit a note
`rnote edit` can be simplified by `rnote e`.\
In order to edit a note, type:
```
$ rnote edit
```
It will prompt you to enter a note name.\
Or
```
$ rnote edit <name>
```
It will search all notes named `<name>` and will prompt you to choose one if multiple notes found. 

### List notes
`rnote list` can be simplified by `rnote ls` or `rnote l`.\
You can list all notes by typing:
```
$ rnote list
```
It will show you all notes and prompt to optionnaly choose one to open.\
You can also list all notes from a `<category>` by typing:
```
$ rnote list -c
``` 
or
```
$ rnote list --category
```
It will prompt you to enter category name.

### Remove notes
`rnote remove` can be simplified by `rnote r` or `rnote rm`.\
To delete a note, you can simply type:
```
$ rnote remove
```
It will prompt you to enter a name of a note.
Or
```
$ rnote remove <name>
```
Both commands will prompt if you do want to delete a note.\
\
You can also remove all notes created at a certain date with:
```
$ rnote remove -d
```
Or 
```
$ rnote remove --date
```
Which will prompt to enter a date in the format `YYYY-mm-dd`.
### Search notes
`rnote search` can be simplified by `rnote s`.\
To search a note simply type:
```
$ rnote search
```
Or
```
$ rnote search <name>
```
First one will prompt you to enter the name.\
\
You can also search by word/string containing in some note:
```
$ rnote search -w
```
or
```
$ rnote search --word
```
It will prompt to enter a string to search in all notes and will prompt you to choose one if multiple notes found.

### Show notes
To show a note, type:
```
$ rnote show
```
Or
```
$ rnote show <name>
```
It will show a note in a scrollable TextView in a raw terminal.\
To scroll use arrow keys or mouse. To quit click on any button.\
\
To show all notes, use flag `--all`:
```
$ rnote show --all
```
Or
```
$ rnote show -a
```
To show all notes from a `<category>`, use flag `--category`:
```
$ rnote show -c
```
Or
```
$ rnote show --category
```
It will prompt you to enter category name.

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
