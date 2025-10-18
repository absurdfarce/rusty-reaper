use std::io::Error;
use futures::prelude::*;

use aws_config::SdkConfig;
use aws_sdk_ec2 as ec2;
use clap::{Parser, Subcommand, Args};

pub mod rr;
use rr::aws;
use rr::subcommands;
use rr::{ImageLang, ImagePlatform};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    // Type of image we wish to operate on
    #[arg(value_enum, short, long)]
    lang: Option<ImageLang>,

    #[arg(value_enum, short, long)]
    platform: Option<ImagePlatform>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    List(ListArgs),
    Delete(DeleteArgs)
}

#[derive(Args)]
struct ListArgs {}

#[derive(Args)]
struct DeleteArgs {}

async fn build_client(config:SdkConfig) -> ec2::Client {
    ec2::Client::new(&config)
}

async fn eval_subcommand(images:Vec<ec2::types::Image>,
                         cmd: &Command,
                         lang: &Option<ImageLang>,
                         platform: &Option<ImagePlatform>) -> Result<(), Error> {
    match &cmd {
        Command::List(_args) => { subcommands::list_command(images, lang, platform) }
        Command::Delete(_args) => { subcommands::delete_command(images, lang, platform) }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    let cli = Cli::parse();

    aws_config::load_from_env()
        .then(|cfg:SdkConfig| { build_client(cfg) })
        .then(|client:ec2::Client| { aws::describe_images(client, &cli.lang, &cli.platform) })
        .then(|images:Vec<ec2::types::Image>| { eval_subcommand(images, &cli.command, &cli.lang, &cli.platform) })
        .await
}