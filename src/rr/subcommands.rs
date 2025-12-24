use anyhow::{anyhow, Result};
use aws_sdk_ec2 as ec2;
use log::{error, info};
use tabled::Table;
use tabled::settings::Style;

use crate::{DeleteArgs, ListArgs};
use crate::rr::aws::{to_lang_string, to_platform_string, deregister_image};
use crate::rr::driverimage::{build_driver_images_by_lang_and_platform};

// CLI subcommands and (where necessary) some helpers

pub async fn list_command(client: &ec2::Client, args: &ListArgs) -> Result<()> {

    match build_driver_images_by_lang_and_platform(&client, &args.lang, &args.platform).await {
        Ok(images) => {
            info!("Listing images for language {}, platform {}", to_lang_string(&args.lang), to_platform_string(&args.platform));
            println!("{}", Table::new(&images).with(Style::blank()));
            Ok(())
        },
        Err(e) =>
            Err(anyhow!("Error retrieving driver images for lang {} and platform {}: {}",
                        to_lang_string(&args.lang),
                        to_platform_string(&args.platform),
                        e))
    }
}

pub async fn delete_command(client: &ec2::Client, args: &DeleteArgs) -> Result<()> {

    info!("Deleting image with ID {}", &args.image_id);
    match deregister_image(client, &args.image_id).await {
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