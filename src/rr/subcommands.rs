use std::io::Error;
use aws_sdk_ec2 as ec2;
use ec2::types::Image;
use crate::{ImageLang, ImagePlatform};
use crate::rr::tabledimage;

pub fn list_command(images:Vec<Image>, _lang: &Option<ImageLang>, _platform: &Option<ImagePlatform>) -> Result<(), Error> {

    tabledimage::print_image_table(images);
    Ok(())
}

pub fn delete_command(_images:Vec<Image>, _lang: &Option<ImageLang>, _platform: &Option<ImagePlatform>) -> Result<(), Error> {

    println!("We're deleting images!");

    Ok(())
}