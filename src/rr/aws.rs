use aws_sdk_ec2 as ec2;
use ec2::types::Image;
use ec2::types::Filter;

// Collection of ops that work on AWS types

// Gather information about AWS images.  Leaks Image type in response.
pub async fn describe_images(client:ec2::Client, filter_string:&str) -> Vec<Image> {
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
