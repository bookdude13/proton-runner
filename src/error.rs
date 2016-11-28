use rustc_serialize::json;
use std::{io, error, fmt};

#[derive(Debug)]
pub enum Error {
    FolderNotEmpty(String, usize),
    InvalidPlaylistItem(String),
    Io(io::Error),
    JsonDecode(json::DecoderError),
    JsonEncode(json::EncoderError),
    MusicError(String),
    MusicFileNotFound(String),
    ProtonCli(String),
    Rsfml(String),
    TodoErr,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::FolderNotEmpty(_, _) => "Root folder was not empty",
            Error::Io(_) => "IO error occurred",
            Error::InvalidPlaylistItem(_) => "Invalid playlist item",
            Error::JsonDecode(_) => "Json decoding error occurred",
            Error::JsonEncode(_) => "Json encoding error occurred",
            Error::MusicError(_) => "Music-related error occurred",
            Error::MusicFileNotFound(_) => "Music file not found",
            Error::ProtonCli(_) => "proton_cli error occurred",
            Error::Rsfml(_) => "Rsfml error occurred",
            Error::TodoErr => "Todo",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::FolderNotEmpty(_, _) => None,
            Error::InvalidPlaylistItem(_) => None,
            Error::Io(ref err) => Some(err),
            Error::JsonDecode(ref err) => Some(err),
            Error::JsonEncode(ref err) => Some(err),
            Error::MusicError(_) => None,
            Error::MusicFileNotFound(_) => None,
            Error::ProtonCli(_) => None,
            Error::Rsfml(_) => None,
            Error::TodoErr => None,
       }
   }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::FolderNotEmpty(ref root, count) => write!(f,
                "{} was not empty: {} files exist", root, count),
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
            Error::ProtonCli(ref description) => write!(f,
                "proton_cli error: {}", description),
            Error::Rsfml(ref description) => write!(f, 
                "Rsfml error: {}", description),
            Error::TodoErr => write!(f, "TodoErr"),
        }
    }
}
