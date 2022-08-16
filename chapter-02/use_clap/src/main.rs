use clap::{ArgEnum, Parser};
use std::path::PathBuf;

#[derive(ArgEnum, Clone, Debug)]
enum ErrorFormats {
    Text,
    Json,
    Xml,
}

/// Example of use of Clap
#[derive(Parser, Debug)]
struct Options {
    /// Activate verbose mode
    #[clap(short = 'V', long = "verbose")]
    verbose: bool,

    /// Set the name of the crate
    #[clap(long = "crate-name")]
    crate_name: Option<String>,

    /// Specify which edition of the compiler to use when compiling code
    #[clap(long = "edition")]
    edition: Option<u16>,

    /// Specify the format used by compiler errors
    #[clap(long = "error-format", arg_enum, default_value_t = ErrorFormats::Text)]
    error_format: ErrorFormats,

    /// List the files to process
    #[clap(name = "FILE", parse(from_os_str))]
    input: Vec<PathBuf>,
}

fn main() {
    print!("{:?}", Options::parse());
}
