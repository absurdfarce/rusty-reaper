use tabled::{Table, Tabled};

use aws_sdk_ec2 as ec2;
use ec2::types::BlockDeviceMapping;
use ec2::types::Image;

// Collection of ops related to generating output via tabled crate

#[derive(Tabled)]
pub struct TabledImage {
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
pub fn from_image(image:&Image) -> TabledImage {

    TabledImage {
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
pub fn from_image_vector(images:Vec<Image>) -> Vec<TabledImage> {
    images.iter().map(from_image).collect()
}

pub fn print_image_table(images:Vec<ec2::types::Image>) {
    let images = from_image_vector(images);
    let table = Table::new(images);
    println!("{}", table);
}