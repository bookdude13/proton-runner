use rustc_serialize::json;
use rserial;
use std::{io, error, fmt};

#[derive(Debug)]
pub enum Error {
    DmxTooLong(usize),
    EmptyData,
    FolderNotEmpty(String, usize),
    InvalidDataLength(u32, u32),
    InvalidPlaylistItem(String),
    Io(io::Error),
    JsonDecode(json::DecoderError),
    JsonEncode(json::EncoderError),
    MusicError(String),
    MusicFileNotFound(String),
    PathNotFound(String),
    ProtonCli(String),
    Rsfml(String),
    Serial(rserial::Error),
    TodoErr,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DmxTooLong(_) => "DMX data too long to send",
            Error::EmptyData => "Empty data vector provided",
            Error::FolderNotEmpty(_, _) => "Root folder was not empty",
            Error::Io(_) => "IO error occurred",
            Error::InvalidDataLength(_, _) => "Data length is invalid",
            Error::InvalidPlaylistItem(_) => "Invalid playlist item",
            Error::JsonDecode(_) => "Json decoding error occurred",
            Error::JsonEncode(_) => "Json encoding error occurred",
            Error::MusicError(_) => "Music-related error occurred",
            Error::MusicFileNotFound(_) => "Music file not found",
            Error::PathNotFound(_) => "Path was not found",
            Error::ProtonCli(_) => "proton_cli error occurred",
            Error::Rsfml(_) => "Rsfml error occurred",
            Error::Serial(_) => "Serial error occurred",
            Error::TodoErr => "Todo",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::DmxTooLong(_) => None,
            Error::EmptyData => None,
            Error::FolderNotEmpty(_, _) => None,
            Error::InvalidDataLength(_, _) => None,
            Error::InvalidPlaylistItem(_) => None,
            Error::Io(ref err) => Some(err),
            Error::JsonDecode(ref err) => Some(err),
            Error::JsonEncode(ref err) => Some(err),
            Error::MusicError(_) => None,
            Error::MusicFileNotFound(_) => None,
            Error::PathNotFound(_) => None,
            Error::ProtonCli(_) => None,
            Error::Rsfml(_) => None,
            Error::Serial(ref err) => Some(err),
            Error::TodoErr => None,
       }
   }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DmxTooLong(ref length) => write!(f,
                "DMX data too long to send. Length: {}", length),
            Error::EmptyData => write!(f,
                "Data provided was empty"),
            Error::FolderNotEmpty(ref root, count) => write!(f,
                "{} was not empty: {} files exist", root, count),
            Error::InvalidDataLength(ref bad, ref good) => write!(f,
                "Data length invalid. Given {}, should be {}", bad, good),
            Error::InvalidPlaylistItem(ref item) => write!(f,
                "Invalid playlist item: {}", item),
            Error::Io(ref err) => write!(f,
                "IO error occurred: {}", error::Error::description(err)),
            Error::JsonDecode(ref err) => write!(f,
                "Json decoding error occurred: {}", err),
            Error::JsonEncode(ref err) => write!(f,
                "Json encoding error occurred: {}", err),
            Error::MusicError(ref description) => write!(f,
                "Music-related error occured: {}", description),
            Error::MusicFileNotFound(ref path) => write!(f,
                "Music file not found at path '{}'", path),
            Error::PathNotFound(ref path) => write!(f,
                "Path not found: {}", path),
            Error::ProtonCli(ref description) => write!(f,
                "proton_cli error: {}", description),
            Error::Rsfml(ref description) => write!(f, 
                "Rsfml error: {}", description),
            Error::Serial(ref err) => write!(f,
                "Serial error occured: {}", err),
            Error::TodoErr => write!(f, "TodoErr"),
        }
    }
}
