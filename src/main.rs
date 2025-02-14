use aws_sdk_ec2 as ec2;
use clap::{Parser, Subcommand, ValueEnum};

pub mod rr;
use rr::aws;
use rr::imagedata;

#[derive(ValueEnum,Clone,Debug)]
enum ImageType {
    Java,
    Python,
    Nodejs,
    CppFocal,
    CppJammy,
    CppRocky8,
    CppRocky9,
    Cpp
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    // Type of image we wish to operate on
    #[arg(value_enum, short, long)]
    image: Option<ImageType>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    List,
    Delete
}

fn filter_string(image_type: &ImageType) -> &str {
    match image_type {
        ImageType::Java => "java-driver-bionic-64-*",
        ImageType::Python => "python-driver-bionic-64-*",
        ImageType::Nodejs => "nodejs-driver-jammy-64-*",
        ImageType::CppFocal => "cpp-driver-focal-64-*",
        ImageType::CppJammy => "cpp-driver-jammy-64-*",
        ImageType::CppRocky8 => "cpp-driver-rocky8-64-*",
        ImageType::CppRocky9 => "cpp-driver-rocky9-64-*",
        ImageType::Cpp => "cpp-driver-*",
    }
}

async fn get_image_data(client:ec2::Client, filter_string: &str) -> Vec<imagedata::ImageData> {

    let images = aws::describe_images(client, filter_string).await;
    images.iter().map(|img| imagedata::to_image_data(img)).collect()
}

#[tokio::main]
async fn main() -> Result<(), ec2::Error> {

    let cli = Cli::parse();

    let config = aws_config::load_from_env().await;
    let client = ec2::Client::new(&config);

    match &cli.command{
        Some(Command::List) => {
            let image_type = cli.image.unwrap_or_else(|| ImageType::Java);
            let filter_string = filter_string(&image_type);
            println!("Retrieving image data, filter string: {}", filter_string);
            for image_data in get_image_data(client, filter_string).await {
                println!("Image: {}", image_data);
            }
        }
        Some(Command::Delete) => {
            println!("Performing deletes");
        }
        None => {}
    }

    Ok(())
}