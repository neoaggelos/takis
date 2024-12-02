use clap::Parser;
use id3::frame::{Picture, PictureType};
use id3::{Tag, TagLike};
use regex::Regex;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fs::{read, rename};
use std::path::Path;

fn format_to_title(a: &str) -> String {
    fn next_char(ch: char, some_prev: Option<char>) -> char {
        let mut this = ch;
        let replacements = HashMap::from([
            ('ά', 'α'),
            ('έ', 'ε'),
            ('ή', 'η'),
            ('ί', 'ι'),
            ('ό', 'ο'),
            ('ύ', 'υ'),
            ('ώ', 'ω'),
            ('Ά', 'Α'),
            ('Έ', 'Ε'),
            ('Ή', 'Η'),
            ('Ί', 'Ι'),
            ('Ό', 'Ο'),
            ('Ύ', 'Υ'),
            ('Ώ', 'Ω'),
        ]);

        if let Some(new) = replacements.get(&ch) {
            this = *new
        }

        if let Some(prev) = some_prev {
            if this.is_lowercase() && prev != '\'' && !prev.is_alphanumeric() {
                this = this.to_ascii_uppercase()
            }
        }

        return this;
    }

    let mut b = String::new();
    a.chars().for_each(|ch| {
        b.push(next_char(ch, b.chars().last()));
    });

    b
}

#[derive(Parser, Debug)]
struct Cli {
    /// Files to update
    files: Vec<String>,

    /// Clear existing tag
    #[arg(long, default_value_t = false)]
    clear: bool,

    /// Set album name
    #[arg(long)]
    album: Option<String>,
    /// Set artist name
    #[arg(long)]
    artist: Option<String>,
    /// Set release year
    #[arg(long)]
    year: Option<i32>,
    /// Set genre
    #[arg(long)]
    genre: Option<String>,

    /// Set album cover
    #[arg(long)]
    cover: Option<String>,

    /// Set track number.
    #[arg(long, group = "track-options")]
    track: Option<u32>,
    /// Set track number using a regex that matches part of the filename.
    #[arg(long, group = "track-options")]
    track_regex: Option<String>,
    /// Set track number and increment on consequent files (using alphabetical order).
    #[arg(long, group = "track-options")]
    track_increment: Option<u32>,

    /// Set song title.
    #[arg(long, group = "title-options")]
    title: Option<String>,
    /// Set song title using a regex that matches part of the filename.
    #[arg(long, group = "title-options")]
    title_regex: Option<String>,

    /// Rename file after applying tag
    #[arg(long, default_value_t = false)]
    rename: bool,
    // TODO: use a template engine and add configurable rename format.
    // /// Rename file format after applying tag
    // #[arg(long, default_value_t = String::from("{track:02} - {title}.mp3"))]
    // rename_format: String,
    /// Extra cover from file and exit
    #[arg(long)]
    extract_cover: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut idx: u32 = 0;
    for file in args.files {
        let path = Path::new(file.as_str());

        let basename = path.file_name().unwrap().to_str().unwrap();
        let basepath = path.parent().unwrap_or(Path::new("."));

        let mut must_update = args.clear;
        let mut tag = match &args.clear {
            true => Tag::new(),
            false => match Tag::read_from_path(&file) {
                Ok(tag) => tag,
                Err(id3::Error {
                    kind: id3::ErrorKind::NoTag,
                    ..
                }) => Tag::new(),
                Err(err) => return Err(Box::new(err)),
            },
        };

        if let Some(extract_to) = &args.extract_cover {
            return match tag.pictures().next() {
                Some(p) => match std::fs::write(extract_to, p.data.clone()) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::<dyn Error>::from(format!(
                        "failed to write cover to '{}': {}",
                        extract_to, err
                    ))),
                },
                None => Err(Box::<dyn Error>::from(format!(
                    "no image found in '{}'",
                    file
                ))),
            };
        }

        match (args.year, tag.year()) {
            (Some(want), Some(have)) if want == have => (),
            (Some(want), _) => {
                tag.set_year(want);
                must_update = true;
            }
            _ => (),
        }

        match (&args.artist, tag.artist()) {
            (Some(want), Some(have)) if want == have => (),
            (Some(want), _) => {
                tag.set_artist(want.as_str());
                must_update = true;
            }
            _ => (),
        }

        match (&args.album, tag.album()) {
            (Some(want), Some(have)) if want == have => (),
            (Some(want), _) => {
                tag.set_album(want.as_str());
                must_update = true;
            }
            _ => (),
        }

        match (&args.genre, tag.genre()) {
            (Some(want), Some(have)) if want == have => (),
            (Some(want), _) => {
                tag.set_genre(want.as_str());
                must_update = true;
            }
            _ => (),
        }

        let some_want_track = if let Some(track) = args.track {
            Some(track)
        } else if let Some(track) = args.track_increment {
            let track = track + idx;
            idx = idx + 1;
            Some(track)
        } else if let Some(track_regex) = &args.track_regex {
            match Regex::new(track_regex)
                .map(|r| r.captures(&basename))?
                .and_then(|caps| caps.name("track"))
                .map(|m| m.as_str().parse::<u32>())
            {
                Some(Ok(track)) => Some(track),
                _ => None,
            }
        } else {
            None
        };

        match (some_want_track, tag.track()) {
            (Some(want), Some(have)) if want == have => (),
            (Some(want), _) => {
                tag.set_track(want);
                must_update = true;
            }
            _ => (),
        }

        let some_want_title = if let Some(title) = &args.title {
            Some(Cow::Borrowed(title))
        } else if let Some(title_regex) = &args.title_regex {
            match Regex::new(title_regex)
                .map(|r| r.captures(&basename))?
                .and_then(|caps| caps.name("title"))
            {
                Some(m) => Some(Cow::Owned(format_to_title(m.as_str()))),
                _ => None,
            }
        } else {
            None
        };

        match (some_want_title, tag.title()) {
            (Some(want), Some(have)) if want.as_str() == have => (),
            (Some(want), _) => {
                tag.set_title(want.as_str());
                must_update = true;
            }
            _ => (),
        }

        if let Some(cover) = &args.cover {
            let b = match read(cover.as_str()) {
                Ok(bytes) => bytes,
                Err(err) => {
                    return Err(Box::<dyn Error>::from(format!(
                        "failed to read cover '{}': {}",
                        cover.as_str(),
                        err
                    )))
                }
            };

            let p = Picture {
                data: b,
                description: "".to_string(),
                mime_type: "image/jpeg".to_string(),
                picture_type: PictureType::CoverFront,
            };

            if !tag.pictures().any(|tp| p == *tp) {
                tag.remove_all_pictures();
                tag.add_frame(p);
                must_update = true
            }
        }

        if must_update {
            match tag.write_to_path(path, id3::Version::Id3v24) {
                Ok(_) => (),
                Err(err) => {
                    return Err(Box::<dyn Error>::from(format!(
                        "failed to update tag for '{}': {}",
                        path.display(),
                        err,
                    )))
                }
            }
        }

        let new_file = match (args.rename, tag.title(), tag.track()) {
            (false, _, _) => file.clone(),
            (true, Some(title), Some(track)) => {
                let v = format!("{track:02} - {title}.mp3");
                basepath.join(v).to_string_lossy().into_owned()
            }
            (true, Some(title), _) => {
                let v = format!("{title}.mp3");
                basepath.join(v).to_string_lossy().into_owned()
            }
            (true, _, _) => file.clone(),
        };

        if new_file != file && !Path::new(&new_file).exists() {
            match rename(&file, &new_file) {
                Ok(_) => (),
                Err(err) => {
                    return Err(Box::<dyn Error>::from(format!(
                        "failed to rename '{}' to '{}': {}",
                        &file, &new_file, err
                    )))
                }
            };
        }

        let mut output = BTreeMap::new();
        tag.frames().for_each(|f| {
            output.insert(
                f.id(),
                if let Some(p) = f.content().picture() {
                    Cow::Owned(format!("<{} bytes>", p.data.len()))
                } else if let Some(text) = f.content().text() {
                    Cow::Borrowed(text)
                } else {
                    Cow::Borrowed("<bytes>")
                },
            );
        });
        println!("{} {:?}", new_file, output);
    }

    Ok(())
}
