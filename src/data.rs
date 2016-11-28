use rustc_serialize::json;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use std::str;

use error::Error;
use types::SequenceData;


/// Get the sequence data from proton_cli
fn get_data(proj_name: &str) -> Result<Vec<SequenceData>, Error> {
    // ./proton_cli get-playlist-data <proj-name>
    let output = try!(Command::new("./proton_cli")
        .arg("get-playlist-data")
        .arg(proj_name)
        .output()
        .map_err(|_| Error::ProtonCli("Failed to run proton_cli process".to_string())));

    // Make sure no error thrown
    let err_str = str::from_utf8(&output.stderr).expect("Data not valid UTF-8");
    if err_str.len() != 0 {
        return Err(Error::ProtonCli(err_str.to_string()));
    }

    // Make sure data starts like JSON
    let plist_data_json = str::from_utf8(&output.stdout).expect("Playlist data not valid UTF-8");
    if &plist_data_json[0..3] != "[[[" {
        return Err(Error::ProtonCli("Returned data not valid: ".to_string() + plist_data_json));
    }

    // get-playlist-data outputs just the JSON playlist data (as of 11/27/2016),
    // so we just grab the output and call it good
    let plist_data_raw: Vec<Vec<Vec<u16>>> = try!(
        json::decode(plist_data_json).map_err(Error::JsonDecode));

    // Make SequenceData objects out of the raw data
    // Give a generic name for now
    let plist_data = plist_data_raw.into_iter().enumerate()
        .map(|(idx, seq_data)| SequenceData {
            name: "Sequence".to_string() + &idx.to_string(),
            data: seq_data
        }).collect::<Vec<SequenceData>>();

    Ok(plist_data)
}

/// Update the local copy of the show's sequence data
pub fn update_data(proj_name: &str) -> Result<(), Error> {
    // Get new data
    let new_data = try!(get_data(proj_name));

    // Make SequenceData directory if it doesn't exist
    let _ = fs::create_dir("SequenceData");

    // Write new data to files for offline use
    for sequence_data in new_data {
        let mut output_path = "SequenceData/".to_string();
        output_path.push_str(&sequence_data.name);
        output_path.push_str(&".json");
        let data_json = try!(json::encode(&sequence_data.data).map_err(Error::JsonEncode));
        try!(File::create(&output_path)
            .and_then(|mut f| f.write(data_json.as_bytes()))
            .map_err(Error::Io));
    }

    Ok(())
}
