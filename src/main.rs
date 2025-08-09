use std::io::Error;
use futures::future;
use futures::prelude::*;

use aws_config::SdkConfig;
use aws_sdk_ec2 as ec2;
use clap::{Parser, Subcommand, Args, ValueEnum};
use strum_macros::Display;

pub mod rr;
use rr::aws;
use rr::subcommands;

#[derive(ValueEnum,Clone,Debug,Display)]
pub enum ImageLang {
    Java,
    Python,
    Nodejs,
    Cpp,
    Csharp,
}

#[derive(ValueEnum,Clone,Debug,Display)]
pub enum ImagePlatform {
    Bionic,
    Focal,
    Jammy,
    Rocky8,
    Rocky9,
    Windows
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    // Type of image we wish to operate on
    #[arg(value_enum, short, long)]
    lang: Option<ImageLang>,

    #[arg(value_enum, short, long)]
    platform: Option<ImagePlatform>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List(ListArgs),
    Delete(DeleteArgs)
}

#[derive(Args)]
struct ListArgs {}

#[derive(Args)]
struct DeleteArgs {}

#[tokio::main]
async fn main() -> Result<(), Error> {

    let cli = Cli::parse();
    let aws_images = aws_config::load_from_env()
        .then(|cfg:SdkConfig| { future::ok(ec2::Client::new(&cfg)) })
        .then(|client:Result<ec2::Client,Error>| { aws::describe_images(client.unwrap(), &cli.lang, &cli.platform) })
        .await;

    match &cli.command {
        Commands::List(_args) => { subcommands::list_command(aws_images, cli.lang, cli.platform) }
        Commands::Delete(_args) => { subcommands::delete_command(aws_images, cli.lang, cli.platform) }
    }
}