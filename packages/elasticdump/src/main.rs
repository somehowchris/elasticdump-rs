use structopt::StructOpt;

pub mod helpers;
pub mod params;
use helpers::bool::toggle_bool;
use helpers::r#type::Type;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "elasticdump",
  about = "Import and export tools for elasticsearch"
)]
struct Opt {
  #[structopt(flatten)]
  input: params::input::InputOpt,

  #[structopt(flatten)]
  output: params::output::OutputOpt,

  #[structopt(flatten)]
  elasticsearch: params::elasticsearch::ElasticsearchOpt,

  #[structopt(flatten)]
  aws: params::aws::AwsOpt,

  #[structopt(long, help = "Overwrite output file if it exists")]
  overwrite: bool,

  #[structopt(
    long,
    short,
    default_value = "100",
    help = "How many objects to move in batch per operation\nlimit is approximate for file streams"
  )]
  limit: u128,

  #[structopt(
    long,
    short,
    default_value = "0",
    help = "How many objects to retrieve\n(0 -> no limit)"
  )]
  size: u128,

  #[structopt(
    long,
    short,
    default_value = "1",
    help = "How many concurrent request is sent to a specified transport"
  )]
  concurrency: u128,

  #[structopt(
    long = "concurrencyInterval",
    default_value = "5000",
    help = "The length of time in milliseconds before the interval count resets. Must be finite."
  )]
  concurrency_interval: u128,

  #[structopt(
    long = "intervalCap",
    default_value = "5",
    help = "The max number of transport request in the given interval of time."
  )]
  interval_cap: u128,

  #[structopt(
    long = "carryoverConcurrencyCount",
    parse(from_flag = toggle_bool),
    help = "Whether the task must finish in the given concurrencyInterval\n(intervalCap will reset to the default whether the request is completed or not)\nor will be carried over into the next interval count,\nwhich will effectively reduce the number of new requests created in the next interval\ni.e. intervalCap -= <num of carried over requests>"
  )]
  carryover_concurrency_count: bool,

  #[structopt(
    long = "throttleInterval",
    default_value = "1",
    help = "The length of time in milliseconds to delay between getting data from an inputTransport and sending it to an outputTransport"
  )]
  throttle_interval: u128,

  #[structopt(
    long = "debug",
    short,
    help = "Display the elasticsearch commands being used"
  )]
  debug: bool,

  #[structopt(
    long = "quiet",
    short,
    help = "Suppress all messages except for errors"
  )]
  quiet: bool,

  #[structopt(
    long = "type",
    short,
    default_value = "Data",
    help = "What are we exporting?"
  )]
  r#type: Type,

  #[structopt(
    long = "delete",
    help = "Delete documents one-by-one from the input as they are\nmoved.  Will not delete the source index"
  )]
  delete: bool,

  #[structopt(
    long = "sourceOnly",
    help = "Output only the json contained within the document _source\nNormal: {\"_index\":\"\",\"_type\":\"\",\"_id\":\"\", \"_source\":{SOURCE}}\nsourceOnly: {SOURCE}"
  )]
  source_only: bool,

  #[structopt(
    long = "ignore-errors",
    help = "Will continue the read/write loop on write error"
  )]
  ignore_errors: bool,

  #[structopt(
    long = "maxSockets",
    default_value = "340282366920938463463374607431768211455",
    help = "How many simultaneous HTTP requests can we process make?"
  )]
  max_sockets: u128,

  #[structopt(
    long = "timeout",
    help = "Integer containing the number of milliseconds to wait for\na request to respond before aborting the request. Passed\ndirectly to the request library. Mostly used when you don't\ncare too much if you lose some data when importing\nbut rather have speed."
  )]
  timeout: Option<u128>,

  #[structopt(
    long = "offset",
    default_value = "0",
    help = "Integer containing the number of rows you wish to skip\nahead from the input transport.  When importing a large\nindex, things can go wrong, be it connectivity, crashes,\nsomeone forgetting to `screen`, etc.  This allows you\nto start the dump again from the last known line written\n(as logged by the `offset` in the output).  Please be\nadvised that since no sorting is specified when the\ndump is initially created, there's no real way to\nguarantee that the skipped rows have already been\nwritten/parsed.  This is more of an option for when\nyou want to get most data as possible in the index\nwithout concern for losing some rows in the process,\nsimilar to the `timeout` option."
  )]
  offset: u128,

  #[structopt(
    long = "toLog",
    parse(from_flag = toggle_bool),
    help = "When using a custom outputTransport, should log lines\nbe appended to the output stream?"
  )]
  to_log: bool,
}

fn main() {
  let opt: Opt = Opt::from_args();
  println!("{:?}", opt);
}
