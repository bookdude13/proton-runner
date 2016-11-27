extern crate proton_runner;
extern crate sfml;

use std::path::Path;

use sfml::audio::{Music, SoundStatus};
use sfml::system::{Time, sleep};

use proton_runner::error::Error;


fn play_music(music_path: &str) -> Result<(), Error> {
    if !Path::new(music_path).exists() {
        return Err(Error::TodoErr);
    }

    let mut music = match Music::new_from_file(music_path) {
        Some(m) => m,
        None => return Err(Error::MusicError("Creating rsfml music object failed".to_string()))
    };

    music.play();

    while music.get_status() == SoundStatus::Playing {
        // Display the playing position
        print!("\rPlaying... {:.2}",
               music.get_playing_offset().as_seconds());
        // Leave some CPU time for other processes
        sleep(Time::with_milliseconds(100));
    }

    Ok(())
}

fn main() {
    match play_music("Music/(1) Jingle Breaks.ogg") {
        Ok(_) => println!("Worked!"),
        Err(e) => println!("{}", e)
    };
}

