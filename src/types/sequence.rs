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
        let (end_clock_tx, end_clock_rx) = mpsc::channel();
        let (debug_tx, debug_rx) = mpsc::channel();
        let (end_debug_tx, end_debug_rx) = mpsc::channel();
        
        // Re-sync music and sequence every x frames
        let check_frame = 50;

        // Spawn timer that ticks once per frame until all frames have been ticked
        let num_frames = data.num_frames;
        let frame_dur = data.frame_dur_ms as u64;
        let music_dur = music.get_duration().as_milliseconds();
        let music_frame_dur = music_dur as f32 / num_frames as f32;

        // Keep track of the frame currently being played (updated by clock and syncing)
        let mut curr_frame = 0;

        // Separate thread for debugging I/O to prevent lag
        let debug_thread = thread::spawn(move || {
            loop {
                // Check to see if told to terminate
                match end_debug_rx.try_recv() {
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                        println!("Terminating.");
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }

                // Print all messages in buffer
                loop {
                    match debug_rx.try_recv() {
                        Ok(difference) => println!("Lag! Frame difference: {}", difference),
                        Err(mpsc::TryRecvError::Disconnected) => {
                            println!("Terminating.");
                            return;
                        }
                        Err(mpsc::TryRecvError::Empty) => { break; }
                    }
                }

                thread::sleep(Duration::from_millis(1000));
            }
        });

        let clock_thread = thread::spawn(move || {
            loop {
                // Check to see if told to terminate
                match end_clock_rx.try_recv() {
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

        // Play music
        music.play();

        // Output every frame (assuming this takes less than frame_dur time)
        for frame in clock_rx.iter() {

            // Sync every so often
            if frame % check_frame == 0 {
                let real_frame = (music.get_playing_offset().as_milliseconds() as f32 / music_frame_dur) as u32;
                if real_frame != curr_frame {
                    match debug_tx.send(real_frame - curr_frame) {
                        Ok(_) => {},
                        Err(_) => println!("Error sending debug info")
                    }

                    curr_frame = real_frame;
                }
            }

            let d = &data.data[frame as usize];
            match dmx.send(d) {
                Ok(_) => (),
                Err(e) => println!("\tError: {}", e),
            }

            // Stop when music done or past last frame
            if music.get_status() == audio::SoundStatus::Stopped || curr_frame >= num_frames {
                // Tell clock thread to stop
                match end_clock_tx.send(0) {
                    Ok(_) => {},
                    Err(_) => { println!("Clock already terminated."); }
                };

                // Tell debug thread to stop
                match end_debug_tx.send(0) {
                    Ok(_) => {},
                    Err(_) => { println!("Debug thread already terminated."); }
                };

                // Let clock thread exit cleanly (wait for it)
                match clock_thread.join() {
                    Ok(_) => {},
                    Err(e) => { println!("Clock thread panicked with error: {:?}", e); },
                };

                // Let debug thread exit cleanly (wait for it)
                match debug_thread.join() {
                    Ok(_) => {},
                    Err(e) => { println!("Debug thread panicked with error: {:?}", e); },
                };


                // Done, so finish
                return Ok(());
            }
        }
        
        println!("Done.");
        Ok(())
    }
}
