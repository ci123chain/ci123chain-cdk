use crate::prelude::{String, ToString};

#[derive(Debug)]
pub enum Error {
    ContractErr {
        msg: &'static str,
    },
    NotFound {
        msg: &'static str,
    },
    ParseErr {
        msg: &'static str,
    },
    SerializeErr {
        msg: &'static str,
    },
    ValidationErr {
        field: &'static str,
        msg: &'static str,
    },
    NullPointer {},
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::ContractErr { msg: m } => "Contract error: ".to_string() + m,
            Error::NotFound { msg: m } => m.to_string() + " not found",
            Error::ParseErr { msg: m } => "Error parsing ".to_string() + m,
            Error::SerializeErr { msg: m } => "Error serializing ".to_string() + m,
            Error::ValidationErr { field: f, msg: m } => "Invalid ".to_string() + f + ": " + m,
            Error::NullPointer {} => "Received null pointer, refuse to use".to_string(),
        }
    }
}
