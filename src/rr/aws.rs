use aws_sdk_ec2 as ec2;
use ec2::types::{Image, Filter};
use crate::{ImageLang, ImagePlatform};

fn to_string(lang:&ImageLang) -> String {
    format!("{}-driver", lang).to_lowercase()
}

fn build_filter_string(lang:&Option<ImageLang>, platform:&Option<ImagePlatform>) -> String {
    let image_type = lang.as_ref().unwrap_or_else(|| &ImageLang::Java);
    match platform {
        None => format!("{}-*", to_string(&image_type).to_string()),
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