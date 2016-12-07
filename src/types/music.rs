use sfml::audio;
use sfml::system::{Time, sleep};

use commands;
use DmxOutput;
use error::Error;
use utils;


pub struct Music {
    music: audio::Music
}

impl Music {

    pub fn run(dmx: &mut DmxOutput, music: &mut audio::Music) -> Result<(), Error> {

        println!("Playing music");

        try!(commands::all_off(dmx));

        // Play music
        music.play();

        // Loop until done playing
        while music.get_status() == audio::SoundStatus::Playing {
            // Leave some CPU time for other processes
            sleep(Time::with_milliseconds(100));
        }

        Ok(())
    }
}
