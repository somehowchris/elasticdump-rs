use std::path::PathBuf;
use structopt::StructOpt;

mod helpers;
use helpers::bool::toggle_bool;
use helpers::json::validate_json;
use helpers::r#type::Type;
use helpers::time::Duration;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "elasticdump",
  about = "Import and export tools for elasticsearch"
)]
struct Opt {
  #[structopt(long, short, help = "Source location (required)")]
  input: String,

  #[structopt(
    long = "input-index",
    default_value = "all",
    help = "Source index and type\n(example: index/type)"
  )]
  input_index: String,

  #[structopt(long, short, help = "Destination location (required)")]
  output: String,

  #[structopt(
    long = "output-index",
    default_value = "all",
    help = "Destination index and type \n(example: index/type)"
  )]
  output_index: String,

  #[structopt(long)]
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
    long = "headers",
    default_value = "{\"User-Agent\": \"elasticdump\"}",
    help = "Add custom headers to Elastisearch requests (helpful when\nyour Elasticsearch instance sits behind a proxy)",
    parse(try_from_str = validate_json)
  )]
  headers: String,

  #[structopt(
    long = "params",
    help = "Add custom parameters to Elastisearch requests uri. Helpful when you for example \nwant to use elasticsearch preference",
    parse(try_from_str = validate_json)
  )]
  params: Option<String>,

  #[structopt(
    long = "searchBody",
    help = "Preform a partial extract based on search results\n(when ES is the input, default values are\nif ES > 5\n\t`'{\"query\": { \"match_all\": {} }, \"stored_fields\": [\"*\"], \"_source\": true }'`\nelse\n\t`'{\"query\": { \"match_all\": {} }, \"fields\": [\"*\"], \"_source\": true }'`",
    parse(try_from_str = validate_json)
  )]
  search_body: Option<String>,

  #[structopt(
    long = "searchWithTemplate",
    help = "Enable to use Search Template when using --searchBody\n\nIf using Search Template then searchBody has to consist of \"id\" field and \"params\" objects\n\nIf \"size\" field is defined within Search Template, it will be overridden by --size parameter\n\nSee https://www.elastic.co/guide/en/elasticsearch/reference/current/search-template.html"
  )]
  search_with_template: bool,

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
    long = "scrollId",
    help = "The last scroll Id returned from elasticsearch.\nThis will allow dumps to be resumed used the last scroll Id &\n`scrollTime` has not expired."
  )]
  scroll_id: Option<String>,

  #[structopt(
    long = "scrollTime",
    default_value = "10m",
    parse(try_from_str = Duration::new),
    help = "Time the nodes will hold the requested search in order."
  )]
  scroll_time: Duration,

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
    long = "noRefresh",
    help = "Disable input index refresh.\nPositive:\n  1. Much increase index speed\n  2. Much less hardware requirements\nNegative:\n  1. Recently added data may not be indexed\nRecommended to use with big data indexing,\nwhere speed and system health in a higher priority\nthan recently added data."
  )]
  no_refresh: bool,

  #[structopt(
    long = "inputTransport",
    help = "Provide a custom js file to use as the input transport",
    parse(from_os_str)
  )]
  input_transport: Option<PathBuf>,

  #[structopt(
    long = "outputTransport",
    help = "Provide a custom js file to use as the output transport",
    parse(from_os_str)
  )]
  output_transport: Option<PathBuf>,

  #[structopt(
    long = "toLog",
    parse(from_flag = toggle_bool),
    help = "When using a custom outputTransport, should log lines\nbe appended to the output stream?"
  )]
  to_log: bool,

  #[structopt(
    long = "awsChain",
    help = "Use https://aws.amazon.com/blogs/security/a-new-and-standardized-way-to-manage-credentials-in-the-aws-sdks/ location and ordering for resolving credentials including environment variables, config files, EC2 and ECS metadata locations\nRecommended option for use with AWS"
  )]
  aws_chain: Option<String>,

  #[structopt(
    long = "awsAccessKeyId",
    help = "When using Amazon Elasticsearch Service protected by\nAWS Identity and Access Management (IAM), provide\nyour Access Key ID"
  )]
  aws_access_key_id: Option<String>,

  #[structopt(
    long = "awsSecretAccessKey",
    help = "When using Amazon Elasticsearch Service protected by\nAWS Identity and Access Management (IAM), provide\nyour Secret Access Key"
  )]
  aws_secret_access_key: Option<String>,

  #[structopt(
    long = "awsIniFileProfile",
    help = "Alternative to --awsAccessKeyId and --awsSecretAccessKey,\nloads credentials from a specified profile in aws ini file.\nFor greater flexibility, consider using --awsChain\nand setting AWS_PROFILE and AWS_CONFIG_FILE\nenvironment variables to override defaults if needed",
    parse(from_os_str)
  )]
  aws_ini_file_profile: Option<PathBuf>,

  #[structopt(
    long = "awsService",
    help = "Sets the AWS service that the signature will be generated for\n(default: calculated from hostname or host)"
  )]
  aws_service: Option<String>,

  #[structopt(
    long = "awsRegion",
    help = "Sets the AWS region that the signature will be generated for\n(default: calculated from hostname or host)"
  )]
  aws_region: Option<String>,

  #[structopt(
    long = "awsUrlRegex",
    default_value = "^https?:\\.*.amazonaws.com.*$",
    help = "Regular expression that defined valied AWS urls that should be signed"
  )]
  aws_url_regex: String,
}

fn main() {
  let opt: Opt = Opt::from_args();
  println!("{:?}", opt);
}
