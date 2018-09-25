use log::SetLoggerError;
use reqwest::Error as ReqwestError;
use select::document::Document;
use std::io::Error as IOError;
use systray::SystrayError;
use toml::de::Error as TomlError;

#[derive(Debug, Deserialize)]
pub struct FreshwallError {
    pub message: String,
}

impl From<ReqwestError> for FreshwallError {
    fn from(error: ReqwestError) -> Self {
        FreshwallError {
            message: String::from("Reqwest Error: ") + &error.to_string(),
        }
    }
}

impl From<Document> for FreshwallError {
    fn from(_error: Document) -> Self {
        FreshwallError {
            message: String::from("HTML Document Error"),
        }
    }
}

impl From<IOError> for FreshwallError {
    fn from(error: IOError) -> Self {
        FreshwallError {
            message: String::from("IO Error: ") + &error.to_string(),
        }
    }
}

impl From<TomlError> for FreshwallError {
    fn from(error: TomlError) -> Self {
        FreshwallError {
            message: String::from("TOML Error: ") + &error.to_string(),
        }
    }
}

impl From<SystrayError> for FreshwallError {
    fn from(error: SystrayError) -> Self {
        FreshwallError {
            message: String::from("Tray Error: ") + &error.to_string(),
        }
    }
}

impl From<SetLoggerError> for FreshwallError {
    fn from(error: SetLoggerError) -> Self {
        FreshwallError {
            message: String::from("SetLogger Error: ") + &error.to_string(),
        }
    }
}
