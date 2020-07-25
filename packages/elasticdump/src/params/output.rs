use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct OutputOpt {
  #[structopt(long = "output", short = "o", help = "Destination location (required)")]
  destination: String,

  #[structopt(
    long = "output-index",
    default_value = "all",
    help = "Destination index and type \n(example: index/type)"
  )]
  index: String,

  #[structopt(
    long = "outputTransport",
    help = "Provide a custom js file to use as the output transport",
    parse(from_os_str)
  )]
  transport: Option<PathBuf>,
}
