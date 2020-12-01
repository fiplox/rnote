# rnote
[![license](https://img.shields.io/badge/licence-GPLv3-blue)](https://framagit.org/fiplox/rpass/-/blob/main/LICENSE)

A minimal note taking cli tool. 

## Description

**rnote** creates `Markdown` text files (with persission set to 600) in a date-named or category-named directories with a name of a header of the note. 

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
