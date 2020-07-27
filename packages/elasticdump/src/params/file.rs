use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "file")]
pub struct FileOpt {
  #[structopt(long, help = "Overwrite output file if it exists")]
  overwrite: bool,

  #[structopt(
    long,
    short,
    default_value = "100",
    help = "How many objects to move in batch per operation\nlimit is approximate for file streams"
  )]
  limit: u128,
}
