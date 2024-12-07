# takis

Command-line tool to manage ID3 tags on mp3 files.

```bash
$ takis --help
```

## Why

- Efficiently manage ID3 tags across my local music library
- Work well with utf-8 tag data, which id3v2 (C) and id3v2-ng (Python) struggle with
- Practice my Rust skills -- Option<>, Result<>, lifetimes and Cow, command-line arguments

The code itself is nothing marvellous to look at, and a more experienced Rust developer would find many things that can be written in a more idiomatic way. If you do, please reach out, would be glad to know more.

## Build

```bash
cargo build --release
install ./target/release/takis /usr/local/bin
```

## Examples

```bash
# extract cover image (if any)
takis --extract-cover cover.jpg file.mp3

# parse files in the format "05 - struggling under words of grief.mp3" (or similar), clear existing tag, extract track number and song title, set album, year, cover and rename files to "05 - Struggling Under Words Of Grief.mp3"
takis --track-regex '.*?(?P<track>\d+).*' --title-regex '.*?\d+(\s\-|\.)?\s(?P<title>.*)\.mp3' --genre 'Metalcore' --artist 'Shot For My Lover' --album 'The Toxin' --year 2005 --cover cover.jpg --clear --rename *.mp3
```
