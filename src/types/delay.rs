use DmxOutput;
use error::Error;

pub struct Delay {
    delay_ms: u64
}

impl Delay {

    pub fn run(delay_ms: u32) -> Result<(), Error> {
        Err(Error::TodoErr)
    }
}
