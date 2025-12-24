use anyhow::{anyhow, Result};
use aws_config::SdkConfig;
use aws_sdk_ec2 as ec2;
use clap::{Parser, Subcommand, Args};
use env_logger::Env;
use futures::prelude::*;

pub mod rr;
use rr::subcommands;
use rr::{ImageLang, ImagePlatform};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {

    List(ListArgs),
    Delete(DeleteArgs)
}

#[derive(Args)]
pub struct ListArgs {

    #[arg(value_enum, short, long)]
    lang: Option<ImageLang>,

    #[arg(value_enum, short, long)]
    platform: Option<ImagePlatform>,
}

#[derive(Args)]
pub struct DeleteArgs {

    #[arg(short, long)]
    image_id: String
}

async fn build_client(config:SdkConfig) -> ec2::Client {
    ec2::Client::new(&config)
}

async fn eval_subcommand(client: &ec2::Client,
                         cmd: &Command) -> Result<()> {
    match &cmd {
        Command::List(args) => subcommands::list_command(client, args).await,
        Command::Delete(args) => subcommands::delete_command(client, args).await
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    // Stolen from env_logger samples (see https://github.com/rust-cli/env_logger/blob/main/examples/default.rs)
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let cli = Cli::parse();

    let client = aws_config::load_from_env()
        .then(|cfg: SdkConfig| { build_client(cfg) })
        .await;

    eval_subcommand(&client, &cli.command).await
}