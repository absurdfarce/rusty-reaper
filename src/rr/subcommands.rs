use anyhow::Result;
use aws_sdk_ec2 as ec2;
use log::{info};
use tabled::Table;
use tabled::settings::Style;

use crate::{ListArgs};
use crate::rr::aws;
use crate::rr::driverimage::DriverImage;

// CLI subcommands and (where necessary) some helpers

pub async fn list_command(_client: &ec2::Client, images:Vec<DriverImage>, args: &ListArgs) -> Result<()> {

    info!("Listing images for language {}, platform {}", aws::to_lang_string(&args.lang), aws::to_platform_string(&args.platform));
    println!("{}", Table::new(&images).with(Style::blank()));
    Ok(())
}

pub async fn delete_command(_client: &ec2::Client) -> Result<()> {

    info!("We're deleting images!");
    Ok(())
}