use aws_sdk_ec2 as ec2;
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

async fn get_image_data(client:ec2::Client) -> Result<ImageData,ec2::Error> {

    let resp = client.describe_images()
        .filters(Filter::builder().name("name").values("cpp-driver-*").build())
        .send()
        .await?;
    let first = resp.images()
        .first()
        .unwrap();
    Ok(ImageData {
        image_id: first.image_id().unwrap().to_string(),
        creation_date: first.creation_date().unwrap().to_string(),
        snapshot_ids: first.block_device_mappings().iter()
            .filter(|mapping| -> bool { !mapping.ebs().is_none() })
            .map(|mapping:&BlockDeviceMapping| -> String {
                mapping.ebs().expect("ebs entry was null")
                .snapshot_id().expect("snapshot_id was null").to_string()
            })
            .collect::<Vec<String>>()
    })
}

#[tokio::main]
async fn main() -> Result<(), ec2::Error> {
    let config = aws_config::load_from_env().await;
    let client = ec2::Client::new(&config);

    let image = get_image_data(client).await?;

    println!("Image: {}", image);
    Ok(())
}