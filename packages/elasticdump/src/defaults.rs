use std::string::String;

pub enum Type {
  Data,
  Settings,
  Analyzer,
  Mapping,
  Alias,
}

pub enum ScrollTimeType {
  Hours,
  Minutes,
  Seconds,
  Milliseconds,
}

pub enum ScrollOutput {
  Ok {
    amount: u128,
    time_type: ScrollTimeType,
  },
  InvalidTime {
    input_time_type: String,
    message: String,
  },
}

pub trait TimeExtraction {
  fn extract_time(&self) -> ScrollOutput;
}

impl TimeExtraction for String {
  fn extract_time(&self) -> ScrollOutput {
    if self.len() <= 1 {
      return ScrollOutput::InvalidTime {
        input_time_type: self.to_owned(),
        message: "hey".to_owned(),
      };
    }
    return ScrollOutput::Ok {
      amount: 1,
      time_type: ScrollTimeType::Hours,
    };
  }
}

pub struct ElasticDumpDefaults {
  size: u128,
  limit: u16,
  offset: u128,
  debug: bool,
  r#type: Type,
  delete: bool,
  max_sockets: u128,
  input: String,
  input_index: String,
  output: String,
  output_index: String,
  no_refresh: bool,
  // inputTransport: (),
  // outputTransport: (),
  // searchBody: (),
  search_with_template: bool,
  headers: bool,
  source_only: bool,
  json_lines: bool,
  // format = '',
  ignore_errors: bool,
  support_big_int: bool,
  // big_int_fields = '',
  // scrollId: (),
  scroll_time: String,
  timeout: u128,
  toLog: bool,
  quiet: bool,
  awsChain: bool,
  awsAccessKeyId: (),
  awsSecretAccessKey: (),
  awsIniFileProfile: (),
  awsService: (),
  awsRegion: (),
  awsUrlRegex: (),
  s3AccessKeyId: (),
  s3SecretAccessKey: (),
  s3Region: (),
  s3Endpoint: (),
  s3SSLEnabled: bool,
  s3ForcePathStyle: bool,
  s3Compress: bool,
  fsCompress: bool,
  awsIniFileName: (),
  sessionToken: (),
  transform: (),
  httpAuthFile: (),
  params: (),
  prefix: String,
  suffix: String,
  retryAttempts: u16,
  customBackoff: bool,
  retryDelayBase: i64,
  retryDelay: i64,
  parseExtraFields: String,
  fileSize: i64,
  cert: (),
  key: (),
  pass: (),
  ca: (),
  tlsAuth: bool,
  input_cert: (),
  input_key: (),
  input_pass: (),
  input_ca: (),
  output_cert: (),
  output_key: (),
  output_pass: (),
  output_ca: (),
  inputSocksProxy: (),
  inputSocksPort: (),
  outputSocksProxy: (),
  outputSocksPort: (),
  concurrency: i16,
  throttleInterval: i16,
  carryoverConcurrencyCount: bool,
  intervalCap: i16,
  concurrencyInterval: i64,
  overwrite: bool,
  handleVersion: bool,
  versionType: (),
}
