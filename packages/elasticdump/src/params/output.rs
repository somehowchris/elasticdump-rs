use crate::helpers::source::{validate_source_path, Source};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "output")]
pub struct OutputOpt {
    #[structopt(
    long = "output",
    short = "o",
    name = "output_destination",
    help = "Destination location (required)",
    parse(try_from_str = validate_source_path)
  )]
    destination: Source,

    #[structopt(
        long = "output-index",
        name = "output_index",
        default_value = "all",
        help = "Destination index and type \n(example: index/type)"
    )]
    index: String,

    #[structopt(
        long = "outputTransport",
        name = "output_transport",
        help = "Provide a custom js file to use as the output transport",
        parse(from_os_str)
    )]
    transport: Option<PathBuf>,
}
