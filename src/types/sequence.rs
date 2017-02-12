
use DmxOutput;
use error::Error;
use types::Runnable;

pub struct Sequence {
	seq_path: String,
	music_path: String
}

impl Sequence {
	pub fn new(seq_path: String, music_path: String) -> Sequence {
		// TODO check if paths exist
		Sequence {
			seq_path: seq_path,
			music_path: music_path
		}

	}
}

impl Runnable for Sequence {
	/// Prepare the playlist item for playing (load data into memory)
	fn prepare(&mut self) -> Result<(), Error> {
		Err(Error::TodoErr)
	}

	/// Run the playlist item
	fn run(&self, dmx: &mut DmxOutput) -> Result<(), Error> {
		Err(Error::TodoErr)
	}
}
