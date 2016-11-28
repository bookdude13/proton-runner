extern crate docopt;
extern crate proton_runner;
extern crate rustc_serialize;
extern crate sfml;

use std::env;

use docopt::Docopt;

use proton_runner::error::Error;


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton_runner update-data <proj-name>
  ./proton_runner run-show <proj-name>
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
    let proj_name = args.arg_proj_name.unwrap();
    proton_runner::runner::run_show(&proj_name)
}

