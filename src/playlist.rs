use error::Error;
use types::{Playlist, PlaylistItem};


pub fn remove_item(proj_name: &str, plist_idx: u32) -> Result<(), Error> {
    // Get current playlist
    let mut plist = try!(Playlist::get_playlist(proj_name));

    // Remove item
    try!(plist.remove_item(plist_idx as usize));

    // Write updated playlist to file
    plist.write_to_file()
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
    try!(plist.insert_item(plist_idx as usize, plist_item));

    // Write updated playlist to file
    plist.write_to_file()
}
