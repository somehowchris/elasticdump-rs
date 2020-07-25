use crate::helpers::{json::validate_json, time::Duration};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ElasticsearchOpt {
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
    long = "noRefresh",
    help = "Disable input index refresh.\nPositive:\n  1. Much increase index speed\n  2. Much less hardware requirements\nNegative:\n  1. Recently added data may not be indexed\nRecommended to use with big data indexing,\nwhere speed and system health in a higher priority\nthan recently added data."
  )]
  no_refresh: bool,
}
