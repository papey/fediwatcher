// Mod get - used to get stats
// Uses
use crate::config::Config;
use reqwest;
use serde_json;
use std::fmt;
use std::string::String;
use tokio::runtime::Runtime;

// Errors
// Define ForgeError
#[derive(Debug, Clone)]
struct ForgeError {}

// implements for ForgeError
impl fmt::Display for ForgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error forging url")
    }
}

#[derive(Debug)]
pub enum GetError {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::error::Error),
    ForgeError,
    IOError(std::io::Error),
}

// implement From
// ReqwestError
impl From<reqwest::Error> for GetError {
    fn from(err: reqwest::Error) -> GetError {
        GetError::ReqwestError(err)
    }
}

// JsonError
impl From<serde_json::Error> for GetError {
    fn from(err: serde_json::Error) -> GetError {
        GetError::SerdeError(err)
    }
}

// IOError
impl From<std::io::Error> for GetError {
    fn from(err: std::io::Error) -> GetError {
        GetError::IOError(err)
    }
}

// Forge api url from kind + url
fn forge_api_url(conf: &Config) -> Option<String> {
    // match if stuff is supported
    match conf.kind.as_str() {
        "mastodon" | "pleroma" => {
            return Some(format!("{}{}", conf.url, "/api/v1/instance"));
        }
        "mastodon_user" | "pleroma_user" => {
            return conf
                .get_user_id()
                .and_then(|uid| Some(format!("{}{}{}", conf.url, "/api/v1/accounts/", uid)))
        }
        "plume" => return Some(format!("{}{}", conf.url, "/nodeinfo/2.0")),
        _ => {
            return None;
        }
    };
}

// Functions - Public
// get_data is used to fetch remote data about a specified config
pub fn get_data(conf: &Config) -> Result<serde_json::Value, GetError> {
    // prepare a tokio runtime
    let mut rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => return Err(GetError::IOError(e)),
    };

    // forge uri
    let uri = match forge_api_url(&conf) {
        Some(uri) => uri,
        None => {
            error!("Error forging url for {}", conf.kind);
            return Err(GetError::ForgeError);
        }
    };

    // get request
    let resp = rt.block_on(reqwest::get(uri.as_str()))?;

    // extract resp to serde_json::Value
    let text = rt.block_on(resp.text());

    text.map_err(|e| GetError::from(e))
        .and_then(|s| serde_json::from_str(s.as_str()).map_err(|e| GetError::from(e)))
}

// Tests
// Tester c'est douter
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::create_test_config;

    #[test]
    fn test_forge_url_ok() {
        // prepare
        let test_ok = create_test_config();

        // launch tests and check results
        match forge_api_url(&test_ok) {
            Some(url) => assert_eq!(url, "https://rage.love/api/v1/instance"),
            None => panic!("Error in forge_url test"),
        };
    }

    #[test]
    #[should_panic]
    fn test_forge_url_nok() {
        // prepare
        let mut test_nok = create_test_config();
        test_nok.kind = "shit".to_string();

        // launch test
        match forge_api_url(&test_nok) {
            Some(_) => assert!(true),
            None => panic!("Kind of config {} is not supported", &test_nok.kind),
        };
    }

    #[test]
    fn test_get_data_ok() {
        // prepare
        let test_ok = create_test_config();

        // launch test and check result
        match get_data(&test_ok) {
            // TODO: better test here
            Ok(data) => assert_ne!(data["stats"]["user_count"], 0),
            Err(e) => panic!(e),
        }
    }

    #[test]
    #[should_panic]
    fn test_get_data_nok() {
        // prepare
        let mut test_nok = create_test_config();
        test_nok.kind = "shit".to_string();

        // launch test
        match get_data(&test_nok) {
            // instance kind not supported
            Ok(_) => assert!(true),
            Err(_) => panic!("Error, this kind of config is not supported"),
        }
    }
}
