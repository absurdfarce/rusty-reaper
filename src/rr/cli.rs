use clap::{Args, Parser, Subcommand};
use crate::{ImageLang, ImagePlatform};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {

    List(ListArgs),
    Delete(DeleteArgs)
}

#[derive(Args)]
pub struct ListArgs {

    #[arg(value_enum, short, long)]
    pub lang: Option<ImageLang>,

    #[arg(value_enum, short, long)]
    pub platform: Option<ImagePlatform>,
}

#[derive(Args)]
pub struct DeleteArgs {

    #[arg(short, long)]
    pub image_id: String
}
