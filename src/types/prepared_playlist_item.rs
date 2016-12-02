use sfml::audio;

use DmxOutput;
use error::Error;
use types::{Delay, Music, Pattern, Sequence, SequenceData};

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
