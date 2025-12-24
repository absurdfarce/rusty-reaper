use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Result};
use aws_sdk_ec2 as ec2;
use ec2::types::Image;
use futures::{FutureExt};
use log::{debug};
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

pub async fn build_driver_images_by_lang_and_platform(client:&ec2::Client, lang: &Option<ImageLang>, platform: &Option<ImagePlatform>) -> Result<Vec<DriverImage>> {

    let rv = aws::describe_images_by_lang_and_platform(client, lang, platform)
        .map(|image_result:Result<Vec<Image>>| {
            image_result.map(|images:Vec<Image>| {
                let driver_images:Vec<_> = images.into_iter().map(|i| { build_driver_image(client, i) }).collect();
                futures::future::join_all(driver_images)
            })
        }).await?;
    Ok(rv.await)
}

pub async fn build_driver_image_by_id(client:&ec2::Client, image_id:String) -> Result<DriverImage> {

    let rv = aws::describe_image_by_id(client, image_id)
        .map(|image_result:Result<Image>| {
            image_result.map(|image:Image| { build_driver_image(client, image) })
        }).await?;
    Ok(rv.await)
}

// Build a DriverImage from an AWS Image instance
async fn build_driver_image(client:&ec2::Client, image:Image) -> DriverImage {

    // Gather all snapshots and generate Snapshot structs from them
    let ec2_snapshots =
        match aws::describe_snapshots(client, aws::get_valid_snapshot_ids(&image)).await {
            Ok(snapshots) => snapshots,
            Err(e) => {
                debug!("Error retrieving snapshots for image with ID {}, returning empty vector: {}", image.image_id().unwrap().to_string(),e);
                Vec::new()
            }
        };

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
