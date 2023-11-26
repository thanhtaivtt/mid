use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MIDError {
    ExecuteProcessError(std::io::Error),
    ParseError(std::string::FromUtf8Error),
    ResultWindowsMidError,
}

impl Error for MIDError {}

impl fmt::Display for MIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MIDError::ExecuteProcessError(e) => {
                write!(f, "Failed to execute process: {}", e)
            }
            MIDError::ParseError(e) => {
                write!(f, "Error converting output to UTF-8: {}", e)
            }
            MIDError::ResultWindowsMidError => {
                write!(f, "Empty result windows machine ID")
            }
        }
    }
}
