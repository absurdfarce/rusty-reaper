use clap::ValueEnum;
use strum_macros::Display;

pub mod driverimage;
pub mod aws;
pub mod subcommands;

#[derive(ValueEnum,Clone,Debug,Display)]
pub enum ImageLang {
    Java,
    Python,
    Nodejs,
    Cpp,
    Csharp,
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
