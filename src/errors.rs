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
        kind: &'static str,
        // #[cfg(feature = "backtraces")]
        // backtrace: snafu::Backtrace,
    },
    #[snafu(display("Error parsing {}", kind))]
    ParseErr {
        kind: &'static str,
        // #[cfg(feature = "backtraces")]
        // backtrace: snafu::Backtrace,
    },
    #[snafu(display("Error serializing {}", kind))]
    SerializeErr {
        kind: &'static str,
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

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub fn contract_err<T>(msg: &'static str) -> Result<T> {
    ContractErr { msg }.fail()
}

pub fn invalid<T>(field: &'static str, msg: &'static str) -> Result<T> {
    ValidationErr { field, msg }.fail()
}