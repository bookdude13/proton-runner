
use DmxOutput;
use error::Error;
use types::Runnable;

pub struct Music {
	music_path: String
}

impl Music {
	pub fn new(music_path: String) -> Music {
		// TODO check if path exists

		Music {
			music_path: music_path
		}
	}
}

impl Runnable for Music {
	/// Prepare the playlist item for playing (load data into memory)
	fn prepare(&mut self) -> Result<(), Error> {
		Err(Error::TodoErr)
	}

	/// Run the playlist item
	fn run(&self, dmx: &mut DmxOutput) -> Result<(), Error> {
		Err(Error::TodoErr)
	}
}
