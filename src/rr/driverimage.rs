use std::fmt::{Display, Formatter};

use aws_sdk_ec2 as ec2;
use ec2::types::Image;
use futures::FutureExt;
use tabled::Tabled;

use crate::{ImageLang, ImagePlatform};
use crate::rr::aws;

// The DriverImage struct (our main representation for data retrieved from AWS) and some utility
// methods leveraging it.
//
// Also provide a translation between AWS SDK types and DriverImage and related structs

pub struct Snapshot {
    snapshot_id: String,
    volume_id: String
}

impl Display for Snapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (volume: {})", self.snapshot_id, self.volume_id)
    }
}

#[derive(Tabled)]
pub struct DriverImage {
    name: String,
    image_id: String,
    creation_date: String,
    #[tabled(display("display_snapshots"))]
    snapshots: Vec<Snapshot>
}

fn display_snapshots(val: &Vec<Snapshot>) -> String {
    val.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",")
}

pub async fn build_driver_images(client:&ec2::Client, lang: &Option<ImageLang>, platform: &Option<ImagePlatform>) -> Vec<DriverImage> {

    aws::describe_images(client, lang, platform)
        .then(|images:Vec<Image>| {
            let driver_images:Vec<_> = images.into_iter().map(|i| { build_driver_image(client, i) }).collect();
            futures::future::join_all(driver_images)
        }).await
}

// Build a DriverImage from an AWS Image instance
pub async fn build_driver_image(client:&ec2::Client, image:Image) -> DriverImage {

    // Gather all snapshots and generate Snapshot structs from them
    let ec2_snapshots = aws::describe_snapshots(client, aws::get_snapshot_ids(&image)).await;

    DriverImage {
        name: image.name().unwrap().to_string(),
        image_id: image.image_id().unwrap().to_string(),
        creation_date: image.creation_date().unwrap().to_string(),
        snapshots: ec2_snapshots.iter().map(|ec2_snap| {
            Snapshot {
                snapshot_id: ec2_snap.snapshot_id.as_ref().unwrap().to_string(),
                volume_id: ec2_snap.volume_id.as_ref().unwrap().to_string()
            }
        }).collect::<Vec<Snapshot>>()
    }
}
