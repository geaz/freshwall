#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate select;
extern crate systray;
extern crate toml;
extern crate wallpaper;

mod settings;
use settings::Settings;

mod error;
mod menu;
mod wallhaven;

fn main() {
    let settings = Settings::load().unwrap();
    wallhaven::check_start(settings).unwrap();
    menu::start_systray().unwrap();
}
