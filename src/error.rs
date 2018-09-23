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
    fn from(_error: ReqwestError) -> Self {
        FreshwallError {
            message: "Error".to_string(),
        }
    }
}

impl From<Document> for FreshwallError {
    fn from(_error: Document) -> Self {
        FreshwallError {
            message: "Error".to_string(),
        }
    }
}

impl From<IOError> for FreshwallError {
    fn from(_error: IOError) -> Self {
        FreshwallError {
            message: "Error".to_string(),
        }
    }
}

impl From<TomlError> for FreshwallError {
    fn from(error: TomlError) -> Self {
        FreshwallError {
            message: error.to_string() + &format!("{:?}", error.line_col()),
        }
    }
}

impl From<SystrayError> for FreshwallError {
    fn from(error: SystrayError) -> Self {
        FreshwallError {
            message: error.to_string(),
        }
    }
}
