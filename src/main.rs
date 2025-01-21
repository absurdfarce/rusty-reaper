use aws_sdk_ec2 as ec2;
use ec2::types::Image;
use ec2::types::Filter;
use ec2::types::BlockDeviceMapping;

use std::fmt;

struct ImageData {
    image_id: String,
    creation_date: String,
    snapshot_ids: Vec<String>
}

impl fmt::Display for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(image_id: {}, creation_date: {}, snapshot IDs: {})", self.image_id, self.creation_date, self.snapshot_ids.join(","))
    }    
}

async fn describe_images(client:ec2::Client, filter_string:&str) -> Vec<Image> {
    let resp = client.describe_images()
        .filters(Filter::builder().name("name").values(filter_string).build())
        .send()
        .await;
    match resp {
        Ok(v) => v.images.unwrap_or_default(),
        Err(e) => {
            eprintln!("Error retrieving image data: {}", e);
            Vec::new()
        }
    }
}

async fn build_image_data(images:Vec<Image>) -> Vec<ImageData> {

    images.iter().map(|image| -> ImageData {
        ImageData {
            image_id: image.image_id().unwrap().to_string(),
            creation_date: image.creation_date().unwrap().to_string(),
            snapshot_ids: image.block_device_mappings().iter()
                .filter(|mapping| -> bool { !mapping.ebs().is_none() })
                .map(|mapping:&BlockDeviceMapping| -> String {
                    mapping.ebs().expect("ebs entry was null")
                        .snapshot_id().expect("snapshot_id was null").to_string()
                })
                .collect::<Vec<String>>()
        }
    }).collect()
}

async fn get_image_data(client:ec2::Client) -> Vec<ImageData> {

    let images = describe_images(client, "cpp-driver-*").await;
    build_image_data(images).await
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