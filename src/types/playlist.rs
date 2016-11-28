use error::Error;

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
}
