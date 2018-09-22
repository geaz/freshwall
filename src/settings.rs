use error::FreshwallError;
use std::fs::File;
use std::io::prelude::*;
use toml::from_str;

#[derive(Debug, Deserialize)]
pub struct Wallpaper {
    pub resolution: String,
    pub ratio: String,
}

#[derive(Debug, Deserialize)]
pub struct Categories {
    pub general: bool,
    pub anime: bool,
    pub people: bool,
}

#[derive(Debug, Deserialize)]
pub struct Purity {
    pub sfw: bool,
    pub sketchy: bool,
}

#[derive(Debug, Deserialize)]
pub struct Proxy {
    pub http: Option<String>,
    pub https: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub wallpaper: Wallpaper,
    pub categories: Categories,
    pub purity: Purity,
    pub proxy: Option<Proxy>,
}

impl Settings {
    pub fn load() -> Result<Self, FreshwallError> {
        let mut contents = String::new();
        File::open("config.toml")?.read_to_string(&mut contents)?;
        Ok(from_str(&contents)?)
    }
}
