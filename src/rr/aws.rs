use aws_sdk_ec2 as ec2;
use ec2::types::{Image, Filter};
use crate::{ImageLang, ImagePlatform};

fn to_string(lang:&ImageLang) -> String {
    format!("{}-driver", lang)
}

fn build_filter_string(lang:&Option<ImageLang>, platform:&Option<ImagePlatform>) -> String {
    let image_type = lang.as_ref().unwrap_or_else(|| &ImageLang::Java);
    match platform {
        None => to_string(&image_type).to_string(),
        Some(p) => format!("{}-{}-64-*", to_string(&image_type), p).to_lowercase(),
    }
}

pub async fn describe_images(client:ec2::Client, lang:&Option<ImageLang>, platform:&Option<ImagePlatform>) -> Vec<Image> {
    let filter_string = build_filter_string(lang, platform);
    println!("Retrieving image data, filter string: {}", filter_string);
    let resp = client.describe_images()
        .filters(Filter::builder().name("name").values(filter_string).build())
        .send()
        .await;
    match resp {
        Ok(v) => v.images.unwrap_or_default(),
        Err(e) => {
            eprintln!("Error retrieving image data: {}", e);
            Vec::new()
        }
    }
}

