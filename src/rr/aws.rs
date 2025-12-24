use anyhow;
use aws_sdk_ec2 as ec2;
use aws_sdk_ec2::types::{BlockDeviceMapping, Filter, Image, Snapshot};
use log::{debug, warn};

use crate::{ImageLang, ImagePlatform};

// AWS ops
//
// Functions in this module are expected to return AWS SDK types.  We'll translate those into something useful
// at higher levels of the abstraction.

// The next few functions are an impl of the naming convention used by our AWS images.
pub fn to_lang_string(lang:&Option<ImageLang>) -> String {
    lang.as_ref().unwrap_or_else(|| &ImageLang::Java).to_string()
}

pub fn to_platform_string(platform:&Option<ImagePlatform>) -> String {
    match platform {
        None => "*".to_string(),
        Some(p) => format!("{}", p),
    }
}

fn build_filter_string(lang:&Option<ImageLang>, platform:&Option<ImagePlatform>) -> String {
    let base = format!("{}-driver-{}", to_lang_string(lang),to_platform_string(platform)).to_lowercase();
    match platform {
        None => base,
        Some(_) => format!("{}-64-*", base).to_lowercase(),
    }
}

pub async fn describe_images_by_lang_and_platform(client:&ec2::Client, lang:&Option<ImageLang>, platform:&Option<ImagePlatform>) -> Result<Vec<Image>, anyhow::Error> {
    let filter_string = build_filter_string(lang, platform);
    debug!("Retrieving image data by lang and platform, filter string: {}", filter_string);
    describe_images(client, Filter::builder().name("name").values(filter_string).build()).await
}

pub async fn describe_image_by_id(client:&ec2::Client, image_id:String) -> Result<Image, anyhow::Error> {
    debug!("Retrieving image data by image ID: {}", &image_id);
    match describe_images(client, Filter::builder().name("image-id").values(&image_id).build()).await {
        Ok(images) => Ok(images.first().unwrap().clone()),
        Err(_) => Err(anyhow::anyhow!("No image found for ID {}",&image_id))
    }
}

pub async fn describe_images(client:&ec2::Client, filter:Filter) -> Result<Vec<Image>, anyhow::Error> {
    let resp = client.describe_images()
        .filters(filter)
        .send()
        .await?;
    Ok(resp.images.unwrap_or_default())
}

pub async fn describe_snapshots(client:&ec2::Client, snapshot_ids:Vec<String>) -> Vec<Snapshot> {

    debug!("Retrieving snapshot data, snapshot_ids: {}", snapshot_ids.join(","));
    let resp = client.describe_snapshots()
        .set_snapshot_ids(Some(snapshot_ids))
        .send()
        .await;
    match resp {
        Ok(v) => v.snapshots.unwrap_or_default(),
        Err(e) => {
            warn!("Error retrieving snapshot data: {}", e);
            Vec::new()
        }
    }
}

pub fn get_valid_snapshot_ids(image:&Image) -> Vec<String> {
    image.block_device_mappings().iter()
        .filter(|mapping| -> bool {
            match mapping.ebs() {
                None => {
                    // Runner images are built with an EBS configuration so make sure to exclude
                    // anybody who might come in via our search without one.
                    warn!("Empty ebs entry for image {}", image.image_id().unwrap());
                    false
                }
                Some(ebs) => {
                    match ebs.snapshot_id() {
                        None => {
                            // Similar to the above; if you don't have a snapshot we aren't interested
                            warn!("Empty snapshot ID for ebs entry for image {}", image.image_id().unwrap());
                            false
                        }
                        Some(_) => true
                    }
                }
            }
        })
        .map(|mapping:&BlockDeviceMapping| -> String {
            mapping.ebs().unwrap().snapshot_id().unwrap().to_string()
        })
        .collect::<Vec<String>>()
}

pub async fn deregister_image(client:&ec2::Client, image_id:&String) -> Result<bool, anyhow::Error> {
    debug!("Deregistering image (and deleting snapshots) with image ID: {}", image_id.to_string());
    let resp = client.deregister_image()
        .set_image_id(Some(image_id.to_string()))
        .delete_associated_snapshots(true)
        .send()
        .await;
    match resp {
        Ok(image_output) => Ok(image_output.r#return.unwrap()),
        Err(e) => Err(anyhow::anyhow!("Error deregistering image: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ImageLang, ImagePlatform};

    #[test]
    fn test_build_filter_string() {
        // Special case: no lang is interpreted as Java
        assert_eq!(build_filter_string(&None, &Some(ImagePlatform::Bionic)), "java-driver-bionic-64-*".to_string());

        // Another special case: missing platform is now just left off
        assert_eq!(build_filter_string(&Some(ImageLang::Java), &None), "java-driver-*".to_string());
        assert_eq!(build_filter_string(&None, &None), "java-driver-*".to_string());

        // Old cases that were supported
        assert_eq!(build_filter_string(&Some(ImageLang::Java), &Some(ImagePlatform::Bionic)), "java-driver-bionic-64-*".to_string());
        assert_eq!(build_filter_string(&Some(ImageLang::Python), &Some(ImagePlatform::Bionic)), "python-driver-bionic-64-*".to_string());
        assert_eq!(build_filter_string(&Some(ImageLang::Nodejs), &Some(ImagePlatform::Jammy)), "nodejs-driver-jammy-64-*".to_string());
        assert_eq!(build_filter_string(&Some(ImageLang::Cpp), &Some(ImagePlatform::Focal)), "cpp-driver-focal-64-*".to_string());
        assert_eq!(build_filter_string(&Some(ImageLang::Cpp), &Some(ImagePlatform::Jammy)), "cpp-driver-jammy-64-*".to_string());
        assert_eq!(build_filter_string(&Some(ImageLang::Cpp), &Some(ImagePlatform::Rocky8)), "cpp-driver-rocky8-64-*".to_string());
        assert_eq!(build_filter_string(&Some(ImageLang::Cpp), &Some(ImagePlatform::Rocky9)), "cpp-driver-rocky9-64-*".to_string());
        assert_eq!(build_filter_string(&Some(ImageLang::Cpp), &None), "cpp-driver-*");
        assert_eq!(build_filter_string(&Some(ImageLang::Cpp), &Some(ImagePlatform::Bionic)), "cpp-driver-bionic-64-*".to_string());
        assert_eq!(build_filter_string(&Some(ImageLang::Cpp), &Some(ImagePlatform::Windows)), "cpp-driver-windows-64-*".to_string());
    }
}