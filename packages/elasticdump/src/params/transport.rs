use crate::helpers::bool::toggle_bool;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "transport")]
pub struct TransportOpt {
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
