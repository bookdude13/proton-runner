use rustc_serialize::json;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use sfml::audio::{Music, SoundStatus};
use sfml::system::{Time, sleep};

use DmxOutput;
use error::Error;
use types::{Playlist, PlaylistItem, SequenceData};
use utils;


fn play_sequence(dmx: &mut DmxOutput, seq_path: &str, seq_music: &str) -> Result<(), Error> {
    println!("Sequence!");
    // Check that paths actually exist
    try!(utils::check_path(seq_path));
    try!(utils::check_path(seq_music));
    play_pattern(dmx, seq_path)
}

fn play_pattern(dmx: &mut DmxOutput, pattern_path: &str) -> Result<(), Error> {
    println!("Pattern!");
    // Check that path actually exists
    try!(utils::check_path(pattern_path));

    println!("\tReading pattern from file");
    // Read in pattern/music-less sequence
    let pattern_json = try!(utils::file_as_string(&pattern_path));
    let pattern: SequenceData = try!(json::decode(&pattern_json).map_err(Error::JsonDecode));

    // Make sure there is data for each frame
    if pattern.data.len() != pattern.num_frames as usize {
        return Err(Error::InvalidDataLength(pattern.data.len() as u32, pattern.num_frames));
    }

    println!("\tRunning...");
    // Create channels for clock thread tx/rx
    let (tx, rx) = mpsc::channel();

    // Spawn timer that ticks once per frame until all frames have been ticked
    let num_frames = pattern.num_frames;
    let frame_dur = pattern.frame_dur_ms as u64;
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
        let data = &pattern.data[frame as usize];
        match dmx.send(data) {
            Ok(_) => (),
            Err(e) => println!("\tError: {}", e),
        }
    }
    println!("Done.");
    Ok(())
}

fn play_music(music_path: &str) -> Result<(), Error> {
    println!("Music!");
    // Check that path actually exists
    try!(utils::check_path(music_path));

    // Create music object
    let mut music = match Music::new_from_file(music_path) {
        Some(m) => m,
        None => return Err(Error::MusicError("Creating rsfml music object failed".to_string()))
    };

    // Play music
    music.play();

    // Loop until done playing
    while music.get_status() == SoundStatus::Playing {
        // Leave some CPU time for other processes
        sleep(Time::with_milliseconds(100));
    }

    Ok(())
}

fn play_delay(delay: u32) -> Result<(), Error> {
    println!("Delay!");
    Err(Error::TodoErr)
}

fn run_item(dmx: &mut DmxOutput, item: PlaylistItem) -> Result<(), Error> {
    match item.path {
        Some(path) => match item.music {
            Some(music) => play_sequence(dmx, &path, &music),
            None => play_pattern(dmx, &path),
        },
        None => match item.music {
            Some(music) => play_music(&music),
            None => play_delay(item.duration.unwrap()), // unwrap is safe because all values cannot be None
        }
    }
}

pub fn run_show(dmx_port: &str, proj_name: &str) -> Result<(), Error> {

    println!("Creating DMX outputter");
    // Create dmx outputter
    let mut dmx = try!(DmxOutput::new(dmx_port));

    println!("Reading playlist");
    // Build playlist file path
    let mut plist_path = "Playlists/".to_string();
    plist_path.push_str(&proj_name);
    plist_path.push_str(&".json");

    // Read playlist
    let plist_json = try!(utils::file_as_string(&plist_path));
    let plist: Playlist = try!(json::decode(&plist_json).map_err(Error::JsonDecode));

    println!("Starting the show!");
    // Play playlist
    for (item_idx, plist_item) in plist.items.into_iter().enumerate() {
        print!("Running item {}...", item_idx);
        try!(run_item(&mut dmx, plist_item));
    }
    Ok(())
}
