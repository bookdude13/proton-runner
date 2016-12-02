use std::fs::File;
use std::io::Write;

use rustc_serialize::json;

use error::Error;
use types::PlaylistItem;
use utils;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Playlist {
    pub name: String,
    pub items: Vec<PlaylistItem>
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

    pub fn insert_item(&mut self, idx: usize, item: PlaylistItem) -> Result<(), Error> {
        if idx > self.items.len() {
            let end = self.items.len();
            // TODO: Make error
            println!("Index cannot be past end of array. Changing index {} to {}", idx, end);
            Ok(self.items.insert(end, item))
        } else {
            Ok(self.items.insert(idx, item))
        }
    }

    pub fn remove_item(&mut self, idx: usize) -> Result<(), Error> {
        if idx > self.items.len() {
            let end = self.items.len();
            // TODO: Make error
            println!("Index cannot be past end of array. Changing index {} to {}", idx, end);
            let _ = self.items.remove(end);
        } else {
            let _ = self.items.remove(idx);
        }
        Ok(())
    }
}
