use anyhow::Result;
use aws_sdk_ec2 as ec2;
use clap::Parser;
use env_logger::Env;

use rusty_reaper::aws::{build_client};
use rusty_reaper::cli::{Cli, Command};
use rusty_reaper::subcommands::{list_command, delete_command};

async fn eval_subcommand(client: &ec2::Client,
                         cmd: &Command) -> Result<()> {
    match &cmd {
        Command::List(args) => list_command(client, args).await,
        Command::Delete(args) => delete_command(client, args).await
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    // Stolen from env_logger samples (see https://github.com/rust-cli/env_logger/blob/main/examples/default.rs)
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let client = build_client().await;
    let cli = Cli::parse();
    eval_subcommand(&client, &cli.command).await
}