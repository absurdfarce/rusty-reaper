use std::io::Error;
use aws_sdk_ec2 as ec2;
use ec2::types::Image;
use tabled::Table;
use crate::{ImageLang, ImagePlatform};
use crate::rr::imagedata;

pub fn list_command(images:Vec<Image>, _lang: Option<ImageLang>, _platform: Option<ImagePlatform>) -> Result<(), Error> {

    println!("We're listing images!");
    let images = imagedata::from_image_vector(images);
    let table = Table::new(images);
    println!("{}", table);

    Ok(())
}

pub fn delete_command(_images:Vec<Image>, _lang: Option<ImageLang>, _platform: Option<ImagePlatform>) -> Result<(), Error> {

    println!("We're deleting images!");

    Ok(())
}