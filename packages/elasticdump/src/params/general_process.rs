use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "general")]
pub struct GeneralProcessOpt {

  #[structopt(
    long = "quiet",
    short,
    help = "Suppress all messages except for errors"
  )]
  quiet: bool,

  #[structopt(
    long = "debug",
    short,
    help = "Display the elasticsearch commands being used"
  )]
  debug: bool,

  #[structopt(
    long = "ignore-errors",
    help = "Will continue the read/write loop on write error"
  )]
  ignore_errors: bool,

  #[structopt(
    long = "support-big-int",
    help = "Support big integer numbers"
  )]
  support_big_int: bool,

  #[structopt(
    long = "big-int-fields",
    help = "Sepcifies a comma-seperated list of fields that should be checked for big-int support",

  // todo needed if flag available
    required_if("support_big_int", "true"),
    use_delimiter = true,
    value_delimiter = ",",
  )]
  big_int_fields: Vec<String>,

  #[structopt(
    long = "retryAttempts",
    help = "Integer indicating the number of times a request should be automatically re-attempted before failing\nwhen a connection fails with one of the following errors `ECONNRESET`, `ENOTFOUND`, `ESOCKETTIMEDOUT`,\nETIMEDOUT`, `ECONNREFUSED`, `EHOSTUNREACH`, `EPIPE`, `EAI_AGAIN`"
  )]
  retry_attempts: Option<u128>,


  #[structopt(
    long = "retryDelay",
    default_value = "5000",
    help = "Integer indicating the back-off/break period between retry attempts (milliseconds)"
  )]
  retry_delay: u128,

  #[structopt(
    long = "parseExtraFields",
    help = "Comma-separated list of meta-fields to be parsed ",
    use_delimiter = true,
    value_delimiter = ",",
  )]
  parse_extra_fields: Option<Vec<String>>,
}
