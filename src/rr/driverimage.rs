use tabled::Tabled;

use aws_sdk_ec2 as ec2;
use ec2::types::BlockDeviceMapping;
use ec2::types::Image;

// The DriverImage struct (our main representation for data retrieved from AWS) and some utility
// methods leveraging it.

#[derive(Tabled)]
pub struct DriverImage {
    name: String,
    image_id: String,
    creation_date: String,
    #[tabled(display("display_snapshot_ids"))]
    snapshot_ids: Vec<String>
}

fn display_snapshot_ids(val: &Vec<String>) -> String {
    val.join(",")
}

// Build a TabledImage from an AWS Image instance
pub fn from_image(image:&Image) -> DriverImage {

    DriverImage {
        name: image.name().unwrap().to_string(),
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
}

// Convenience function to apply the transform above to an existing vector
pub fn from_image_vector(images:Vec<Image>) -> Vec<DriverImage> {
    images.iter().map(from_image).collect()
}
