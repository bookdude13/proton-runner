use std::thread;
use std::time::Duration;

use commands;
use DmxOutput;
use error::Error;

pub struct Delay {
    delay_ms: u64
}


impl Delay {

    pub fn run(dmx: &mut DmxOutput, delay_ms: u32) -> Result<(), Error> {
    	println!("Playing delay");

    	try!(commands::all_off(dmx));
        thread::sleep(Duration::from_millis(delay_ms as u64));
        
        Ok(())
    }
}
