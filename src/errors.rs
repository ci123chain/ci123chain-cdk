use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Contract error: {}", msg))]
    ContractErr {
        msg: &'static str,
        // #[cfg(feature = "backtraces")]
        // backtrace: snafu::Backtrace,
    },
    #[snafu(display("{} not found", kind))]
    NotFound {
        kind: String,
        // #[cfg(feature = "backtraces")]
        // backtrace: snafu::Backtrace,
    },
    #[snafu(display("Error parsing {}", kind))]
    ParseErr {
        kind: String,
        // #[cfg(feature = "backtraces")]
        // backtrace: snafu::Backtrace,
    },
    #[snafu(display("Error serializing {}", kind))]
    SerializeErr {
        kind: String,
        // #[cfg(feature = "backtraces")]
        // backtrace: snafu::Backtrace,
    },
    #[snafu(display("Invalid {}: {}", field, msg))]
    ValidationErr {
        field: &'static str,
        msg: &'static str,
        // #[cfg(feature = "backtraces")]
        // backtrace: snafu::Backtrace,
    },
    #[snafu(display("Received null pointer, refuse to use"))]
    NullPointer {},
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::ParseErr {
            kind: err.to_string(),
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub fn contract_err<T>(msg: &'static str) -> Result<T> {
    ContractErr{ msg }.fail()
}

pub fn invalid<T>(field: &'static str, msg: &'static str) -> Result<T> {
    ValidationErr{ field, msg }.fail()
}