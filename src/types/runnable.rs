use DmxOutput;
use error::Error;

/// Interface for all types that can be prepped and run
pub trait Runnable {
	/// Prepare the item for running (load data into memory)
	fn prepare(&mut self) -> Result<(), Error>;

	/// Run the item
	fn run(&self, dmx: &mut DmxOutput) -> Result<(), Error>;
}
