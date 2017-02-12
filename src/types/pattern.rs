
use DmxOutput;
use error::Error;
use types::Runnable;

pub struct Pattern {
	seq_path: String
}

impl Pattern {
	pub fn new(seq_path: String) -> Pattern {
		// TODO check if path exists
		Pattern {
			seq_path: seq_path
		}

	}
}

impl Runnable for Pattern {
	/// Prepare the playlist item for playing (load data into memory)
	fn prepare(&mut self) -> Result<(), Error> {
		Err(Error::TodoErr)
	}

	/// Run the playlist item
	fn run(&self, dmx: &mut DmxOutput) -> Result<(), Error> {
		Err(Error::TodoErr)
	}
}
