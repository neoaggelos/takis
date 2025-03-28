# takis

Command-line tool to manage ID3 tags on mp3 files.

```bash
$ takis --help
```

## Why

- Efficiently manage ID3 tags across my local music library
- Work well with utf-8 tag data, which id3v2 (C) and id3v2-ng (Python) struggle with
- Work well with APIC cover images, which I had troubles with when using eyeD3 (Python)
- Practice my Rust skills -- Option<>, Result<>, lifetimes and Cow, command-line arguments

The code itself is nothing marvellous to look at, and a more experienced Rust developer would find many things that can be written in a more idiomatic way. If you do, please reach out, would be glad to know more.

## Build

```bash
$ cargo build --release
$ install ./target/release/takis /usr/local/bin
```

## Examples

```bash
# extract cover image (if any)
$ takis --extract-cover cover.jpg file.mp3

# parse files in the format "05 - struggling under words of grief.mp3" (or similar), clear existing tag, extract track number and song title, set album, year, cover and rename files to "05 - Struggling Under Words Of Grief.mp3"
$ takis --track-regex '.*?(?P<track>\d+).*' --title-regex '.*?\d+(\s\-|\.)?\s(?P<title>.*)\.mp3' --genre 'Metalcore' --artist 'Shot For My Lover' --album 'The Toxin' --year 2005 --cover cover.jpg --clear --rename *.mp3

# remove id3 tags with id "PRIV" from all mp3 files
$ takis --clear-id PRIV *.mp3
```

## Print ID3 tag

When no arguments are set, takis simply prints the ID3 tags in a table format

```
$ takis ./*.mp3
 TRACK  TITLE                       ARTIST  ALBUM          YEAR  COVER                       GENRE     FILE                                   OTHER
 1      Waiting On A Twist Of Fate  Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./01 - Waiting On A Twist Of Fate.mp3  []
 2      Landmines                   Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./02 - Landmines.mp3                   []
 3      I Can't Wait                Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./03 - I Can't Wait.mp3                []
 4      Time Won't Wait             Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./04 - Time Won't Wait.mp3             []
 5      Future Primitive            Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./05 - Future Primitive.mp3            []
 6      Dopamine                    Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./06 - Dopamine.mp3                    []
 7      Not Quite Myself            Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./07 - Not Quite Myself.mp3            []
 8      Bad Mistake                 Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./08 - Bad Mistake.mp3                 []
 9      Johnny Libertine            Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./09 - Johnny Libertine.mp3            []
 10     Radio Silence               Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./10 - Radio Silence.mp3               []
 11     Preparasi A Salire          Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./11 - Preparasi A Salire.mp3          []
 12     Rise Up                     Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./12 - Rise Up.mp3                     []
 13     Stranger In These Times     Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./13 - Stranger In These Times.mp3     []
 14     I Don't Need Anyone         Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./14 - I Don't Need Anyone.mp3         []
 15     Over The Edge               Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./15 - Over The Edge.mp3               []
 16     House Of Liars              Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./16 - House Of Liars.mp3              []
 17     You Wanted War              Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./17 - You Wanted War.mp3              []
 18     Paint It Black              Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./18 - Paint It Black.mp3              []
 19     It's All Me                 Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./19 - It's All Me.mp3                 []
 20     How The End Begins          Sum 41  Heaven X Hell  2024  <image/jpeg (53890 bytes)>  Punk Pop  ./20 - How The End Begins.mp3          []
```
