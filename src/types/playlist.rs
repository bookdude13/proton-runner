use std::fs::File;
use std::io::Write;

use rustc_serialize::json;
use sfml::audio;

use DmxOutput;
use error::Error;
use types::{Delay, Music, Pattern, Sequence, SequenceData};
use utils;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Playlist {
    pub name: String,
    pub items: Vec<PlaylistItem>
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct PlaylistItem {
    pub path: Option<String>,
    pub music: Option<String>,
    pub duration: Option<u32>
}

pub struct PreparedPlaylistItem {
    pub data: Option<SequenceData>,
    pub music: Option<audio::Music>,
    pub duration: Option<u32>
}

impl Playlist {
    pub fn write_to_file(&self) -> Result<(), Error> {
        // Build playlist file path
        let plist_path = Playlist::get_path(&self.name);

        // Write playlist to file
        let plist_json = try!(json::encode(self).map_err(Error::JsonEncode));
        let _ = try!(File::create(&plist_path)
            .and_then(|mut f| f.write(plist_json.as_bytes()))
            .map_err(Error::Io));

        Ok(())
    }

    fn get_path<'a>(proj_name: &'a str) -> String {
        let mut playlist_path = String::from("Playlists/");
        playlist_path.push_str(proj_name);
        playlist_path.push_str(".json");
        playlist_path
    }

    pub fn get_playlist(proj_name: &str) -> Result<Playlist, Error> {
        // Get path to file
        let playlist_path = Playlist::get_path(proj_name);

        // Load playlist from file
        let plist_json = try!(utils::file_as_string(&playlist_path));

        // Return decoded project
        json::decode(&plist_json).map_err(Error::JsonDecode)
    }

    pub fn add_item(
        proj_name: &str,
        plist_idx: u32,
        path: Option<String>,
        music: Option<String>,
        duration: Option<u32>
    ) -> Result<(), Error> {
        // Get current playlist
        let mut plist = try!(Playlist::get_playlist(proj_name));

        // Create item to add to playlist
        let plist_item = try!(PlaylistItem::new(path, music, duration));

        // Add to playlist items
        try!(plist.insert_into_playlist(plist_idx as usize, plist_item));

        // Write updated playlist to file
        plist.write_to_file()
    }

    fn insert_into_playlist(&mut self, idx: usize, item: PlaylistItem) -> Result<(), Error> {
        if idx > self.items.len() {
            let end = self.items.len();
            println!("Index cannot be past end of array. Changing index {} to {}", idx, end);
            Ok(self.items.insert(end, item))
        } else {
            Ok(self.items.insert(idx, item))
        }
    }
}

impl PreparedPlaylistItem {
    pub fn run(self, dmx: &mut DmxOutput) -> Result<(), Error> {
        if self.data.is_some() && self.music.is_some() {
            Sequence::run(dmx, &self.data.unwrap(), &mut self.music.unwrap())
        } else if self.data.is_some() {
            Pattern::run(dmx, &self.data.unwrap())
        } else if self.music.is_some() {
            Music::run(&mut self.music.unwrap())
        } else {
            Delay::run(self.duration.unwrap())
        }
    }
}

impl PlaylistItem {
    pub fn new(
        path: Option<String>,
        music: Option<String>,
        duration: Option<u32>
    ) -> Result<PlaylistItem, Error> {

        if path.is_none() && music.is_none() && duration.is_none() {
            Err(Error::InvalidPlaylistItem("All values cannot be None".to_string()))
        } else {
            Ok(PlaylistItem {
                path: path,
                music: music,
                duration: duration
            })
        }
    }

    pub fn prepare(&self) -> Result<PreparedPlaylistItem, Error> {
        match self.path {
            Some(ref p) => match self.music {
                Some(ref m) => {
                    let data = try!(utils::load_sequence_data(p));
                    let music = match audio::Music::new_from_file(m) {
                        Some(mm) => mm,
                        None => return Err(Error::MusicError("Creating rsfml music object failed".to_string()))
                    };
                    Ok(PreparedPlaylistItem {
                        data: Some(data),
                        music: Some(music),
                        duration: self.duration
                    })
                },
                None => {
                    let data = try!(utils::load_sequence_data(p));
                    Ok(PreparedPlaylistItem {
                        data: Some(data),
                        music: None::<audio::Music>,
                        duration: self.duration
                    })
                }
            },
            None => match self.music {
                Some(ref m) => {
                    let music = match audio::Music::new_from_file(m) {
                        Some(mm) => mm,
                        None => return Err(Error::MusicError("Creating rsfml music object failed".to_string()))
                    };
                    Ok(PreparedPlaylistItem {
                        data: None::<SequenceData>,
                        music: Some(music),
                        duration: self.duration
                    })
                },
                None => {
                    Ok(PreparedPlaylistItem {
                        data: None::<SequenceData>,
                        music: None::<audio::Music>,
                        duration: self.duration
                    })
                }
            }
        }        
    }
}
