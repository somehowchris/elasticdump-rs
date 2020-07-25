use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct InputOpt {
  #[structopt(long = "input", short = "i", help = "Source location (required)")]
  destination: String,

  #[structopt(
    long = "input-index",
    default_value = "all",
    help = "Source index and type\n(example: index/type)"
  )]
  index: String,

  #[structopt(
    long = "inputTransport",
    help = "Provide a custom js file to use as the input transport",
    parse(from_os_str)
  )]
  transport: Option<PathBuf>,
}
