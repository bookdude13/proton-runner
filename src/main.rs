extern crate docopt;
extern crate proton_runner;
extern crate rustc_serialize;
extern crate sfml;

use std::{env, io};

use docopt::Docopt;

use proton_runner::error::Error;
use proton_runner::types::Show;


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton_runner update-data <proj-name>
  ./proton_runner run-show <proj-name> <dmx-port>
  ./proton_runner (-h | --help)

Options:
  -h --help     Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_proj_name: Option<String>,
    arg_dmx_port: Option<String>,
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

