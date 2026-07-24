use clap::ValueEnum;
use strum_macros::Display;

pub mod aws;
pub mod cli;
pub mod driverimage;
pub mod subcommands;

#[derive(ValueEnum,Clone,Debug,Display)]
pub enum ImageLang {
    Java,
    Python,
    Nodejs,
    Cpp,
    Csharp,
}

pub fn string_to_lang(lang:String)->Result<ImageLang,String>{
    ImageLang::from_str(&lang, true)
}

#[derive(ValueEnum,Clone,Debug,Display)]
pub enum ImagePlatform {
    Bionic,
    Focal,
    Jammy,
    Rocky8,
    Rocky9,
    Windows
}

pub fn string_to_platform(platform:String)->Result<ImagePlatform,String>{
    ImagePlatform::from_str(&platform, true)
}
