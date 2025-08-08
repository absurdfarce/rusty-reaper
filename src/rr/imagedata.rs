use std::fmt;

use tabled::Tabled;

use aws_sdk_ec2 as ec2;
use ec2::types::BlockDeviceMapping;
use ec2::types::Image;

// Collection of ops related to ImageData and it's uses

#[derive(Tabled)]
pub struct ImageData {
    name: String,
    image_id: String,
    creation_date: String,
    #[tabled(display("display_snapshot_ids"))]
    snapshot_ids: Vec<String>
}

impl fmt::Display for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(image_id: {}, name: {}, creation_date: {}, snapshot IDs: {})",
               self.image_id, self.name, self.creation_date, self.snapshot_ids.join(","))
    }
}

fn display_snapshot_ids(val: &Vec<String>) -> String {
    val.join(",")
}

// Build an ImageData instance from an AWS Image instance
pub fn from_image(image:&Image) -> ImageData {

    ImageData {
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
pub fn from_image_vector(images:Vec<Image>) -> Vec<ImageData> {
    images.iter().map(from_image).collect()
}