use std::io::Error;
use futures::prelude::*;

use aws_config::SdkConfig;
use aws_sdk_ec2 as ec2;
use clap::{Parser, Subcommand, Args};

pub mod rr;
use crate::rr::driverimage::{build_driver_images, DriverImage};
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

async fn eval_subcommand(client: &ec2::Client,
                         images:Vec<DriverImage>,
                         cmd: &Command,
                         lang: &Option<ImageLang>,
                         platform: &Option<ImagePlatform>) -> Result<(), Error> {
    match &cmd {
        Command::List(_args) => { subcommands::list_command(client, images, lang, platform) }
        Command::Delete(_args) => { subcommands::delete_command(client, images, lang, platform) }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    let cli = Cli::parse();

    let client = aws_config::load_from_env()
        .then(|cfg:SdkConfig| { build_client(cfg) }).await;

    build_driver_images(&client, &cli.lang, &cli.platform)
        .then(|images:Vec<DriverImage>| { eval_subcommand(&client, images, &cli.command, &cli.lang, &cli.platform) })
        .await
}