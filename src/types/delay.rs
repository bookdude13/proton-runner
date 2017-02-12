
use DmxOutput;
use error::Error;
use types::Runnable;

pub struct Delay {
	duration: u32
}

impl Delay {
	pub fn new(duration: u32) -> Delay {
		Delay {
			duration: duration
		}
	}
}

impl Runnable for Delay {
	/// Prepare the playlist item for playing (load data into memory)
	fn prepare(&mut self) -> Result<(), Error> {
		Err(Error::TodoErr)
	}

	/// Run the playlist item
	fn run(&self, dmx: &mut DmxOutput) -> Result<(), Error> {
		Err(Error::TodoErr)
	}
}
