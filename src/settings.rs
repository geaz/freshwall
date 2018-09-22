use error::FreshwallError;
use std::fs::File;
use std::io::prelude::*;
use toml::from_str;

#[derive(Debug, Deserialize)]
pub struct Wallpaper {
    pub resolution: Option<String>,
    pub ratio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Categories {
    pub general: Option<bool>,
    pub anime: Option<bool>,
    pub people: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Purity {
    pub sfw: Option<bool>,
    pub sketchy: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Proxy {
    pub http: Option<String>,
    pub https: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub wallpaper: Option<Wallpaper>,
    pub categories: Option<Categories>,
    pub purity: Option<Purity>,
    pub proxy: Option<Proxy>,
}

impl Settings {
    pub fn load() -> Result<Self, FreshwallError> {
        let mut contents = String::new();
        File::open("config.toml")?.read_to_string(&mut contents)?;
        Ok(from_str(&contents)?)
    }
}
