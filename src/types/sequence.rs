use sfml::audio;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use DmxOutput;
use error::Error;
use types::SequenceData;


pub struct Sequence;

impl Sequence {

    pub fn run(dmx: &mut DmxOutput, data: &SequenceData, music: &mut audio::Music) -> Result<(), Error> {
        println!("Running sequence");

        // Create channels for clock thread tx/rx and termination
        let (clock_tx, clock_rx) = mpsc::channel();
        let (end_tx, end_rx) = mpsc::channel();

        // Re-sync music and sequence every x frames
        let check_frame = 10;

        // Spawn timer that ticks once per frame until all frames have been ticked
        let num_frames = data.num_frames;
        let frame_dur = data.frame_dur_ms as u64;
        let music_dur = music.get_duration().as_milliseconds();
        let music_frame_dur = music_dur as f32 / num_frames as f32;
        let mut curr_frame = 0;

        // Play music
        music.play();

        let clock_thread = thread::spawn(move || {
            loop {
                // Check to see if told to terminate
                match end_rx.try_recv() {
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                        println!("Terminating.");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                // Tick curr_frame, dying if receiving end terminated
                match clock_tx.send(curr_frame) {
                    Ok(_) => {},
                    Err(_) => { println!("Terminating."); }
                };

                // Sleep for frame duration
                thread::sleep(Duration::from_millis(frame_dur));
                
                // Increment current frame
                curr_frame += 1;
            }
        });

        // Output every frame (assuming this takes less than frame_dur time)
        for frame in clock_rx.iter() {
            let d = &data.data[frame as usize];
            match dmx.send(d) {
                Ok(_) => (),
                Err(e) => println!("\tError: {}", e),
            }

            // Stop when music done or past last frame
            if music.get_status() == audio::SoundStatus::Stopped || curr_frame >= num_frames {
                // Tell clock thread to stop
                match end_tx.send(0) {
                    Ok(_) => {},
                    Err(_) => { println!("Clock already terminated."); }
                };

                // Let clock thread exit cleanly (wait for it)
                match clock_thread.join() {
                    Ok(_) => {},
                    Err(e) => { println!("Clock thread panicked with error: {:?}", e); },
                };

                // Done, so finish
                return Ok(());
            }

            // Sync every so often
            if frame % check_frame == 0 {
                let real_frame = (music.get_playing_offset().as_milliseconds() as f32 / music_frame_dur) as u32;
                if real_frame != curr_frame {
                    println!("Lag! Real frame: {}, curr_frame: {}", real_frame, curr_frame);
                    // curr_frame = real_frame + 1;
                }
            }
        }
        
        println!("Done.");
        Ok(())
    }
}
