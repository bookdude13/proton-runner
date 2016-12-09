use sfml::audio;
use std::thread;
use std::time::Duration;

use DmxOutput;
use error::Error;
use types::SequenceData;


pub struct Sequence;

impl Sequence {

    pub fn run(dmx: &mut DmxOutput, data: &SequenceData, music: &mut audio::Music) -> Result<(), Error> {
        println!("Running sequence");

        let num_frames = data.num_frames;
        let music_dur = music.get_duration().as_milliseconds();
        let music_frame_dur = music_dur as f32 / num_frames as f32;

        // Play music
        music.play();

        loop {            
            let frame = (music.get_playing_offset().as_milliseconds() as f32 / music_frame_dur) as u32;

            let d = &data.data[frame as usize];
            match dmx.send(d) {
                Ok(_) => (),
                Err(e) => println!("\tError: {}", e),
            }

            // Stop when music done or past last frame
            if music.get_status() == audio::SoundStatus::Stopped {
                break;
            }

            // Sleep for frame duration
            thread::sleep(Duration::from_millis(10));
        }

        println!("Done.");
        Ok(())
    }
}
