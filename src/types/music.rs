use sfml::audio;
use std::thread;
use std::time::Duration;

use DmxOutput;
use error::Error;
use types::Runnable;

pub struct Music {
	music: audio::Music
}

impl Music {
	pub fn new(music_path: String) -> Result<Music, Error> {
		// TODO check if path exists
        let music = match audio::Music::from_file(&music_path) {
            Some(mm) => mm,
            None => return Err(Error::MusicError("Creating rsfml music object failed".to_string()))
        };

		Ok(Music {
			music: music
		})
	}
}

impl Runnable for Music {
	/// Run the playlist item
	#[allow(unused_variables)]
	fn run(self: Box<Self>, dmx: &mut DmxOutput) -> Result<(), Error> {
		println!("Playing music");
		let mut music = self.music;

        // Play music
        music.play();

        // Loop until done playing
        while music.status() == audio::SoundStatus::Playing {
            // Leave some CPU time for other processes
            thread::sleep(Duration::from_millis(15));
        }

        Ok(())
	}
}
