extern crate docopt;
extern crate proton_runner;
extern crate rustc_serialize;
extern crate sfml;

use std::env;
use std::path::Path;

use docopt::Docopt;
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

const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton_runner update-data <proj-name>
  ./proton_runner run-show
  ./proton_runner (-h | --help)

Options:
  -h --help     Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_proj_name: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    // Below unwrap()'s are safe within Docopt's usage rules

    let command: fn(Args) -> Result<(), Error> = match env::args().nth(1).unwrap().as_ref() {
        "update-data" => run_update_data,
        "run-show" => run_run_show,
        _ => panic!("Invalid first argument"),
    };

    let result = command(args);
    match result {
        Ok(_) => println!("Worked!"),
        Err(e) => println!("{:?}", e.to_string()),
    };
}

fn run_update_data(args: Args) -> Result<(), Error> {
    let proj_name = args.arg_proj_name.unwrap();
    proton_runner::data::update_data(&proj_name)
}

fn run_run_show(args: Args) -> Result<(), Error> {
    proton_runner::runner::run_show()
}

