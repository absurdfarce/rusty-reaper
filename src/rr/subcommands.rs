use std::io::Error;

use aws_sdk_ec2 as ec2;
use tabled::Table;

use crate::{ImageLang, ImagePlatform};
use crate::rr::aws;
use crate::rr::driverimage::DriverImage;

// CLI subcommands and (where necessary) some helpers

pub fn list_command(_client: &ec2::Client, images:Vec<DriverImage>, lang: &Option<ImageLang>, platform: &Option<ImagePlatform>) -> Result<(), Error> {

    println!("Listing images for language {}, platform {}", aws::to_lang_string(lang), aws::to_platform_string(platform));
    println!("{}", Table::new(images));

    Ok(())
}

pub fn delete_command(_client: &ec2::Client, _images:Vec<DriverImage>, _lang: &Option<ImageLang>, _platform: &Option<ImagePlatform>) -> Result<(), Error> {

    println!("We're deleting images!");

    Ok(())
}