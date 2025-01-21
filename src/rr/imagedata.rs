use std::fmt;

use aws_sdk_ec2 as ec2;
use ec2::types::BlockDeviceMapping;
use ec2::types::Image;

pub struct ImageData {
    image_id: String,
    creation_date: String,
    snapshot_ids: Vec<String>
}

impl fmt::Display for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(image_id: {}, creation_date: {}, snapshot IDs: {})", self.image_id, self.creation_date, self.snapshot_ids.join(","))
    }
}

// Build an ImageData instance from an AWS Image instance
pub fn build_image_data(images:Vec<Image>) -> Vec<ImageData> {

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
