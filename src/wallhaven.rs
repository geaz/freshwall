use reqwest::{Client, Proxy};
use wallpaper;

use select::document::Document;
use select::predicate::Class;

use std::env::temp_dir;
use std::fs::File;

use error::FreshwallError;
use settings::{Proxy as ProxySettings, Settings};

pub fn set_random_wallpaper(settings: Settings) -> Result<(), FreshwallError> {
    let client = get_client(settings.proxy)?;
    let response = client.get("https://alpha.wallhaven.cc/search?q=&categories=100&purity=100&atleast=3440x1440&ratios=21x9&sorting=random&order=desc").send()?;
    let document = Document::from_read(response);
    for node in document.unwrap().find(Class("thumb")).take(1) {
        let wallpaper_url = format!(
            "https://wallpapers.wallhaven.cc/wallpapers/full/wallhaven-{}.jpg",
            node.attr("data-wallpaper-id").unwrap()
        );

        let mut dl_response = client.get(&wallpaper_url).send().unwrap();
        let mut fname: String;
        let mut dest = {
            let filename = "wallpaper.jpg";
            let path = temp_dir().join(filename);
            println!("will be located under: '{:?}'", path);
            fname = path.to_str().to_owned().unwrap().into();
            File::create(path).unwrap()
        };
        dl_response.copy_to(&mut dest).unwrap();
        drop(dest);
        println!("setting to: '{:?}'", fname);
        wallpaper::set_from_path(&fname).unwrap();
    }

    Ok(())
}

fn get_client(proxy_settings: Option<ProxySettings>) -> Result<Client, FreshwallError> {
    let mut client_builder = Client::builder();
    if let Some(proxy_settings) = proxy_settings {
        if let Some(http_proxy) = proxy_settings.http {
            client_builder = client_builder.proxy(Proxy::http(&http_proxy)?);
        }
        if let Some(https_proxy) = proxy_settings.https {
            client_builder = client_builder.proxy(Proxy::https(&https_proxy)?);
        }
    }
    Ok(client_builder.build()?)
}
