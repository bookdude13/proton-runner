use rustc_serialize::json;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use DmxOutput;
use error::Error;
use types::{Delay, Music, Pattern, Playlist, PlaylistItem, PreparedPlaylistItem, Sequence, SequenceData};
use utils;


pub struct Show {
    playlist: Vec<PreparedPlaylistItem>,
    dmx: DmxOutput
}

impl Show {
    pub fn new(dmx_port: &str, proj_name: &str) -> Result<Show, Error> {
        
        println!("Creating DMX outputter");
        // Create dmx outputter
        let dmx = try!(DmxOutput::new(dmx_port));

        println!("Reading playlist");
        // Build playlist file path
        let mut plist_path = "Playlists/".to_string();
        plist_path.push_str(&proj_name);
        plist_path.push_str(&".json");

        // Read playlist
        let plist_json = try!(utils::file_as_string(&plist_path));
        let plist: Playlist = try!(json::decode(&plist_json).map_err(Error::JsonDecode));

        println!("Prepping the show");
        // Setup playlist items
        let prepped_show = plist.items.into_iter()
            .map(|show_item| match show_item.prepare() {
                Ok(prepped) => prepped,
                Err(e) => { panic!(e); }
            })
            .collect::<Vec<PreparedPlaylistItem>>();

        Ok(Show {
            dmx: dmx,
            playlist: prepped_show
        })
    }

    pub fn run(mut self) -> Result<(), Error> {

        // Run show
        for show_item in self.playlist.into_iter() {
            let _ = try!(show_item.run(&mut self.dmx));
        }

        Ok(())
    }
}
