//#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate select;
extern crate toml;
extern crate wallpaper;

mod settings;
use settings::Settings;

mod error;
mod wallhaven;

fn main() {
    let settings = Settings::load().unwrap();
    wallhaven::set_random_wallpaper(settings).unwrap();
}
