use rustc_serialize::json;

use error::Error;
use types::{Playlist, PlaylistItem};
use utils;


fn play_sequence(seq_path: &str, seq_music: &str) -> Result<(), Error> {
    Err(Error::TodoErr)
}

fn play_pattern(pattern_path: &str) -> Result<(), Error> {
    Err(Error::TodoErr)
}

fn play_music(mus_path: &str) -> Result<(), Error> {
    Err(Error::TodoErr)
}

fn play_delay(delay: u32) -> Result<(), Error> {
    Err(Error::TodoErr)
}

fn run_item(item: PlaylistItem) -> Result<(), Error> {
    match item.path {
        Some(path) => match item.music {
            Some(music) => play_sequence(&path, &music),
            None => play_pattern(&path),
        },
        None => match item.music {
            Some(music) => play_music(&music),
            None => play_delay(item.duration.unwrap()), // unwrap is safe because all values cannot be None
        }
    }
}

pub fn run_show(proj_name: &str) -> Result<(), Error> {
    // Build playlist file path
    let mut plist_path = "Playlists/".to_string();
    plist_path.push_str(&proj_name);
    plist_path.push_str(&".json");
    // Read playlist
    let plist_json = try!(utils::file_as_string(&plist_path));
    let plist: Playlist = try!(json::decode(&plist_json).map_err(Error::JsonDecode));
    // Play playlist
    for plist_item in plist.items {
        try!(run_item(plist_item));
    }
    Ok(())
}
