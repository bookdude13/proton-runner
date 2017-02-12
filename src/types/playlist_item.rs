
use error::Error;
use types::{Delay, Music, Pattern, Runnable, Sequence};

/// Mapping to JSON playlist item
#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
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

    pub fn to_runnable(&mut self) -> Box<Runnable> {
        let me = self.clone();
        if me.path.is_some() && me.music.is_some() {
            let mut runnable = Sequence::new(me.path.unwrap(), me.music.unwrap());
            runnable.prepare().expect("Error preparing runnable");
            Box::new(runnable)
        } else if me.path.is_some() {
            let mut runnable = Pattern::new(me.path.unwrap());
            runnable.prepare().expect("Error preparing runnable");
            Box::new(runnable)
        } else if me.music.is_some() {
            let mut runnable = Music::new(me.music.unwrap());
            runnable.prepare().expect("Error preparing runnable");
            Box::new(runnable)
        } else {
            let mut runnable = Delay::new(me.duration.unwrap());
            runnable.prepare().expect("Error preparing runnable");
            Box::new(runnable)
        }
    }
}
