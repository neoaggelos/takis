use clap::Parser;
use id3::frame::{Picture, PictureType};
use id3::{Tag, TagLike};
use regex::Regex;
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

        if let Some(artist) = &args.artist {
            tag.set_artist(artist.as_str());
            must_update = true
        }
        if let Some(album) = &args.album {
            tag.set_album(album.as_str());
            must_update = true
        }
        if let Some(year) = args.year {
            tag.set_year(year);
            must_update = true
        }
        if let Some(genre) = &args.genre {
            tag.set_genre(genre);
            must_update = true
        }
        if let Some(track) = args.track {
            tag.set_track(track);
            must_update = true
        }
        if let Some(track) = args.track_increment {
            tag.set_track(track + idx);
            idx += 1;
            must_update = true
        }
        if let Some(track_regex) = &args.track_regex {
            if let Ok(r) = Regex::new(&track_regex) {
                if let Some(caps) = r.captures(&basename) {
                    if let Some(cap) = caps.name("track") {
                        if let Ok(n) = cap.as_str().parse::<u32>() {
                            tag.set_track(n);
                            must_update = true
                        }
                    }
                }
            }
        }

        if let Some(title) = &args.title {
            tag.set_title(title)
        }
        if let Some(title_regex) = &args.title_regex {
            if let Ok(r) = Regex::new(&title_regex) {
                if let Some(caps) = r.captures(&basename) {
                    if let Some(cap) = caps.name("title") {
                        tag.set_title(format_to_title(cap.as_str()));
                        must_update = true
                    }
                }
            }
        }

        if let Some(cover) = &args.cover {
            let b = match read(cover.as_str()) {
                Ok(bytes) => bytes,
                Err(err) => return Err(Box::new(err)),
            };

            tag.add_frame(Picture {
                data: b,
                description: "".to_string(),
                mime_type: "image/jpeg".to_string(),
                picture_type: PictureType::CoverFront,
            });
            must_update = true
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
            output.insert(f.id(), f.content().text().map_or("<bytes>", |x| x));
        });
        println!("{} {:?}", new_file, output);
    }

    Ok(())
}
