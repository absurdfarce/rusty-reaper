use std::io::Error;
use aws_sdk_ec2 as ec2;
use ec2::types::Image;
use tabled::Table;
use crate::rr::{ImageLang, ImagePlatform};
use crate::rr::driverimage::from_image_vector;

// CLI subcommands and (where necessary) some helpers

pub fn list_command(images:Vec<Image>, _lang: &Option<ImageLang>, _platform: &Option<ImagePlatform>) -> Result<(), Error> {

    let images = from_image_vector(images);
    println!("{}", Table::new(images));

    Ok(())
}

pub fn delete_command(_images:Vec<Image>, _lang: &Option<ImageLang>, _platform: &Option<ImagePlatform>) -> Result<(), Error> {

    println!("We're deleting images!");

    Ok(())
}