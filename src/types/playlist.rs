use sfml::audio;

use DmxOutput;
use error::Error;
use types::{Delay, Music, Pattern, Sequence, SequenceData};
use utils;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Playlist {
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
