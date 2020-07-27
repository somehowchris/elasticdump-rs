use std::path::PathBuf;
use structopt::StructOpt;
use crate::helpers::source::{validate_source_path, Source};

#[derive(Debug, StructOpt)]
#[structopt(name = "input")]
pub struct InputOpt {
  #[structopt(
    long = "input",
    short = "i",
    name = "input_destination",
    help = "Source location (required)",
    parse(try_from_str = validate_source_path)
  )]
  destination: Source,

  #[structopt(
    long = "input-index",
    name = "input_index",
    default_value = "all",
    help = "Source index and type\n(example: index/type)"
  )]
  index: String,

  #[structopt(
    long = "inputTransport",
    name = "input_transport",
    help = "Provide a custom js file to use as the input transport",
    parse(from_os_str)
  )]
  transport: Option<PathBuf>,
}
