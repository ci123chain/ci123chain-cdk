use std::string::ToString;

#[derive(Debug)]
pub enum Error {
    ContractErr {
        msg: &'static str,
    },
    NotFound {
        kind: String,
    },
    ParseErr {
        kind: String,
    },
    SerializeErr {
        kind: String,
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
            Error::ContractErr { msg: e } => format!("Contract error: {}", e),
            Error::NotFound { kind: e } => format!("{} not found", e),
            Error::ParseErr { kind: e } => format!("Error parsing {}", e),
            Error::SerializeErr { kind: e } => format!("Error serializing {}", e),
            Error::ValidationErr { field: f, msg: e } => format!("Invalid {}: {}", f, e),
            Error::NullPointer {} => format!("Received null pointer, refuse to use"),
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
