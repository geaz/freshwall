use error::FreshwallError;
use systray;

use settings::Settings;
use wallhaven;

pub fn start_systray() -> Result<(), FreshwallError> {
    let mut app = systray::Application::new()?;
    app.set_icon_from_file(&String::from("wallpaper.ico"))?;
    app.set_tooltip(&"Freshwall".to_string())?;

    app.add_menu_item(&"Get fresh!".to_string(), |_| {
        let settings = Settings::load().unwrap();
        wallhaven::set_random_wallpaper(settings).unwrap();
    })?;
    app.add_menu_separator()?;
    app.add_menu_item(&"Exit".to_string(), |window| {
        window.quit();
    })?;

    app.wait_for_message();
    Ok(())
}
