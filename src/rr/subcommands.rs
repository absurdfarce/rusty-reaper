use anyhow::Result;
use aws_sdk_ec2 as ec2;
use log::{error, info};
use tabled::Table;
use tabled::settings::Style;

use crate::{DeleteArgs, ListArgs};
use crate::rr::aws;
use crate::rr::driverimage::DriverImage;

// CLI subcommands and (where necessary) some helpers

pub async fn list_command(_client: &ec2::Client, images:Vec<DriverImage>, args: &ListArgs) -> Result<()> {

    info!("Listing images for language {}, platform {}", aws::to_lang_string(&args.lang), aws::to_platform_string(&args.platform));
    println!("{}", Table::new(&images).with(Style::blank()));
    Ok(())
}

pub async fn delete_command(client: &ec2::Client, args: &DeleteArgs) -> Result<()> {

    info!("Deleting image with ID {}", &args.image_id);
    match aws::deregister_image(client, &args.image_id).await {
        Ok(result) => {
            println!("Result of image deletion: {} ", result);
            Ok(())
        },
        Err(e) => {
            error!("Error deleting image: {}", e);
            Err(e)
        }
    }
}