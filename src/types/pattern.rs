use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use DmxOutput;
use error::Error;
use types::SequenceData;
use utils;

pub struct Pattern {
    pattern: SequenceData
}

impl Pattern {
    
    pub fn run(dmx: &mut DmxOutput, data: &SequenceData) -> Result<(), Error> {

        println!("Running pattern");

        // Create channels for clock thread tx/rx
        let (tx, rx) = mpsc::channel();

        // Spawn timer that ticks once per frame until all frames have been ticked
        let num_frames = data.num_frames;
        let frame_dur = data.frame_dur_ms as u64;
        let mut curr_frame = 0;
        thread::spawn(move || {
            while curr_frame != num_frames {
                // TODO maybe map the unwrap error to Error type
                tx.send(curr_frame).unwrap();
                curr_frame += 1;
                thread::sleep(Duration::from_millis(frame_dur));
            }
            
        });

        // Output every frame
        for frame in rx.iter() {
            let d = &data.data[frame as usize];
            match dmx.send(d) {
                Ok(_) => (),
                Err(e) => println!("\tError: {}", e),
            }
        }
        println!("Done.");
        Ok(())
    }
}
