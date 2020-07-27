use structopt::StructOpt;

pub mod helpers;
pub mod params;

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

  #[structopt(flatten)]
  file: params::file::FileOpt,

  #[structopt(flatten)]
  general: params::general_process::GeneralProcessOpt,

  #[structopt(flatten)]
  transport: params::transport::TransportOpt,
}

fn main() {
  let opt: Opt = Opt::from_args();
  println!("{:?}", opt);
}
