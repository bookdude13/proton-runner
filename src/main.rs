extern crate docopt;
extern crate proton_runner;
extern crate rustc_serialize;
extern crate sfml;

use std::{env, io};

use docopt::Docopt;

use proton_runner::DmxOutput;
use proton_runner::error::Error;
use proton_runner::types::Show;


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton_runner allOn <dmx-port>
  ./proton_runner allOff <dmx-port>
  ./proton_runner set <dmx-chan> (on|off) <dmx-port>
  ./proton_runner rangeOn <chan-start> <chan-end> <dmx-port>
  ./proton_runner rangeOff <chan-start> <chan-end> <dmx-port>
  ./proton_runner run-show <proj-name> <dmx-port>
  ./proton_runner update-data <proj-name>
  ./proton_runner (-h | --help)

Options:
  -h --help     Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_chan_start: Option<u32>,
    arg_chan_end: Option<u32>,
    arg_dmx_chan: Option<u32>,
    arg_dmx_port: Option<String>,
    arg_on: Option<bool>,
    arg_off: Option<bool>,
    arg_proj_name: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    // Below unwrap()'s are safe within Docopt's usage rules

    let command: fn(Args) -> Result<(), Error> = match env::args().nth(1).unwrap().as_ref() {
        "allOn" => run_all_on,
        "allOff" => run_all_off,
        "set" => run_set,
        "rangeOn" => run_range_on,
        "rangeOff" => run_range_off,
        "run-show" => run_run_show,
        "update-data" => run_update_data,
        _ => panic!("Invalid first argument"),
    };

    let result = command(args);
    match result {
        Ok(_) => println!("Worked!"),
        Err(e) => println!("{:?}", e.to_string()),
    };
}

fn run_all_on(args: Args) -> Result<(), Error> {
    let dmx_port = args.arg_dmx_port.unwrap();
    
    let mut dmx = try!(DmxOutput::new(&dmx_port));
    
    proton_runner::commands::range_on(&mut dmx, 1, 512)
}

fn run_all_off(args: Args) -> Result<(), Error> {
    let dmx_port = args.arg_dmx_port.unwrap();
    
    let mut dmx = try!(DmxOutput::new(&dmx_port));
    
    proton_runner::commands::range_off(&mut dmx, 1, 512)
}

fn run_range_on(args: Args) -> Result<(), Error> {
    let dmx_port = args.arg_dmx_port.unwrap();
    let chan_start = args.arg_chan_start.unwrap();
    let chan_end = args.arg_chan_end.unwrap();
    
    let mut dmx = try!(DmxOutput::new(&dmx_port));
    let start = dmx_bounded(chan_start);
    let end = dmx_bounded(chan_end);

    proton_runner::commands::range_on(&mut dmx, start, end)
}

fn run_range_off(args: Args) -> Result<(), Error> {
    let dmx_port = args.arg_dmx_port.unwrap();
    let chan_start = args.arg_chan_start.unwrap();
    let chan_end = args.arg_chan_end.unwrap();
    
    let mut dmx = try!(DmxOutput::new(&dmx_port));
    let start = dmx_bounded(chan_start);
    let end = dmx_bounded(chan_end);

    proton_runner::commands::range_off(&mut dmx, start, end)
}

fn run_run_show(args: Args) -> Result<(), Error> {
    // Prepare show
    let proj_name = args.arg_proj_name.unwrap();
    let dmx_port = args.arg_dmx_port.unwrap();
    let show = try!(Show::new(&proj_name, &dmx_port));
    println!("Ready!");

    // Wait for user to run
    let mut input = String::new();
    try!(io::stdin().read_line(&mut input).map_err(Error::Io));
    match input.trim() {
        "run" => show.run(),
        "quit" => Ok(()),
        _ => Ok(println!("Invalid command (must be run or quit)"))
    }
}

fn run_set(args: Args) -> Result<(), Error> {
    println!("{:?}", args);

    let dmx_port = args.arg_dmx_port.unwrap();
    let dmx_chan = args.arg_dmx_chan.unwrap();
    
    let mut dmx = try!(DmxOutput::new(&dmx_port));

    match env::args().nth(1).unwrap().as_ref() {
        "on" => proton_runner::commands::range_on(&mut dmx, dmx_chan, dmx_chan),
        "off" => proton_runner::commands::range_off(&mut dmx, dmx_chan, dmx_chan),
        _ => Ok(println!("This will never happen"))
    }
}

fn run_update_data(args: Args) -> Result<(), Error> {
    let proj_name = args.arg_proj_name.unwrap();
    proton_runner::data::update_data(&proj_name)
}

/// Bind value to range [1, 512]
fn dmx_bounded(unbounded: u32) -> u32 {
    if unbounded < 1 { 1 }
    else if unbounded > 512 { 512 }
    else { unbounded }
}
