use std::path::PathBuf;

use crate::helpers::regex::build_regex;
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct AwsOpt {
    #[structopt(
        long = "awsRegion",
        help = "Sets the AWS region that the signature will be generated for\n(default: calculated from hostname or host)"
    )]
    aws_region: Option<String>,

    #[structopt(
    long = "awsUrlRegex",
    default_value = "^https?:\\.*.amazonaws.com.*$",
    parse(try_from_str = build_regex),
    help = "Regular expression that defined valied AWS urls that should be signed"
  )]
    url_regex: Regex,

    #[structopt(
        long = "awsChain",
        help = "Use https://aws.amazon.com/blogs/security/a-new-and-standardized-way-to-manage-credentials-in-the-aws-sdks/ location and ordering for resolving credentials including environment variables, config files, EC2 and ECS metadata locations\nRecommended option for use with AWS"
    )]
    chain: Option<String>,

    #[structopt(
        long = "awsAccessKeyId",
        help = "When using Amazon Elasticsearch Service protected by\nAWS Identity and Access Management (IAM), provide\nyour Access Key ID"
    )]
    access_key_id: Option<String>,

    #[structopt(
        long = "awsSecretAccessKey",
        help = "When using Amazon Elasticsearch Service protected by\nAWS Identity and Access Management (IAM), provide\nyour Secret Access Key"
    )]
    secret_access_key: Option<String>,

    #[structopt(
        long = "awsIniFileProfile",
        help = "Alternative to --awsAccessKeyId and --awsSecretAccessKey,\nloads credentials from a specified profile in aws ini file.\nFor greater flexibility, consider using --awsChain\nand setting AWS_PROFILE and AWS_CONFIG_FILE\nenvironment variables to override defaults if needed",
        parse(from_os_str)
    )]
    ini_file_profile: Option<PathBuf>,

    #[structopt(
        long = "awsService",
        help = "Sets the AWS service that the signature will be generated for\n(default: calculated from hostname or host)"
    )]
    service: Option<String>,
}
