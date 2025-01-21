use aws_sdk_ec2 as ec2;

pub mod rr;
use rr::aws;
use rr::imagedata;

async fn get_image_data(client:ec2::Client) -> Vec<imagedata::ImageData> {

    let images = aws::describe_images(client, "cpp-driver-*").await;
    imagedata::build_image_data(images)
}

#[tokio::main]
async fn main() -> Result<(), ec2::Error> {
    let config = aws_config::load_from_env().await;
    let client = ec2::Client::new(&config);

    for image_data in get_image_data(client).await {
        println!("Image: {}", image_data);
    }

    Ok(())
}