use std::{io, error, fmt};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    MusicError(String),
    MusicFileNotFound(String),
    Rsfml(String),
    TodoErr,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "IO error occurred",
            Error::MusicError(_) => "Music-related error occurred",
            Error::MusicFileNotFound(_) => "Music file not found",
            Error::Rsfml(_) => "Rsfml error occurred",
            Error::TodoErr => "Todo",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
           Error::Io(ref err) => Some(err),
           Error::MusicError(_) => None,
           Error::MusicFileNotFound(_) => None,
           Error::Rsfml(_) => None,
           Error::TodoErr => None,
       }
   }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f,
                "IO error occurred: {}", error::Error::description(err)),
            Error::MusicError(ref description) => write!(f,
                "Music-related error occured: {}", description),
            Error::MusicFileNotFound(ref path) => write!(f,
                "Music file not found at path '{}'", path),
            Error::Rsfml(ref description) => write!(f, 
                "Rsfml error: {}", description),
            Error::TodoErr => write!(f, "TodoErr"),
        }
    }
}
