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

fn play_music(music_path: &str) -> Result<(), Error> {
    // Check that path actually exists
    if !Path::new(music_path).exists() {
        return Err(Error::PathNotFound(music_path.to_string()));
    }

    // Create music object
    let mut music = match Music::new_from_file(music_path) {
        Some(m) => m,
        None => return Err(Error::MusicError("Creating rsfml music object failed".to_string()))
    };

    // Play music
    music.play();

    // Loop until done playing
    while music.get_status() == SoundStatus::Playing {
        // Leave some CPU time for other processes
        sleep(Time::with_milliseconds(100));
    }

    Ok(())
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
