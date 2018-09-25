#![windows_subsystem = "windows"]

#[macro_use]
extern crate log;
extern crate simplelog;

#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate select;
extern crate systray;
extern crate toml;
extern crate wallpaper;

use simplelog::*;
use std::fs::File;

mod settings;
use settings::Settings;

mod error;
use error::FreshwallError;

mod menu;
mod wallhaven;

fn main() -> Result<(), FreshwallError> {
    init_logger()?;

    let settings = Settings::load()?;
    wallhaven::check_start(settings)?;

    menu::start_systray()?;
    Ok(())
}

fn init_logger() -> Result<(), FreshwallError> {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("freshwall.log")?,
    )])?;
    Ok(())
}
