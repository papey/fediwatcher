// Mod config - used to parse config files
// Uses
use serde::Deserialize;
use std::{fmt, fs, io, vec::Vec};
use url::Url;

// Const
// array of supported kinds
pub const SUPPORTED: [&str; 5] = [
    "mastodon",
    "mastodon_user",
    "pleroma",
    "pleroma_user",
    "plume",
];

// Errors
// Define NotSupportedError
#[derive(Debug, Clone)]
struct NotSupportedError {}

// implements for NotSupportedError
impl fmt::Display for NotSupportedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error kind of config not supported")
    }
}

// Define NoConfigError
struct NoConfigError {}

// implements for NoConfigError
impl fmt::Display for NoConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No config file found")
    }
}

// Define UrlError
#[derive(Debug, Clone)]
struct UrlError {}

// implements for UrlError
impl fmt::Display for UrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error url is not valid")
    }
}

// Define ConfigError
#[derive(Debug)]
pub enum ConfigError {
    IOError(std::io::Error),
    TomlError(toml::de::Error),
    NotSupportedError,
    UrlError(url::ParseError),
    NoConfigError,
}

// implement from
// IOError
impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> ConfigError {
        ConfigError::IOError(err)
    }
}

// TomlError
impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> ConfigError {
        ConfigError::TomlError(err)
    }
}

// UrlError
impl From<url::ParseError> for ConfigError {
    fn from(err: url::ParseError) -> ConfigError {
        ConfigError::UrlError(err)
    }
}

// Structs - public
// Options struct found in config files
#[derive(Deserialize, Debug, Clone)]
pub struct Options {
    user_id: Option<String>,
}

// Struct Config represent data read from conf.d files
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    // name of the config
    pub name: String,
    // url endpoint
    pub url: String,
    // kind
    pub kind: String,
    // options
    pub options: Option<Options>,
}

// Implement new method for config
impl Config {
    #[allow(dead_code)]
    pub fn new(name: String, url: String, kind: String, _options: Option<Options>) -> Config {
        Config {
            name,
            url,
            kind,
            options: None,
        }
    }

    pub fn get_user_id(&self) -> Option<String> {
        self.options.as_ref().and_then(|o| o.user_id.clone())
    }
}

// Implement Display trait for config
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {}, url: {}, kind {}",
            self.name, self.url, self.kind
        )?;

        // if there is an user id, print it
        match self.get_user_id() {
            Some(i) => write!(f, ", options [user_id: {}]", i),
            None => write!(f, ""),
        }
    }
}

// Functions - private
// Read config file and create config struct
fn read_config_file(path: &std::path::PathBuf) -> Result<Config, ConfigError> {
    // what is the current file ?
    info!("Reading {} file", path.display());

    // try to read content
    let content = fs::read_to_string(&path)?;

    // debug file content
    debug!("Content of file {}, is : \n{}", &path.display(), &content);

    // try desirializing toml file into config struct
    let config: Config = toml::from_str(&content)?;

    // debug struct deserializing
    debug!("{}", &config);

    // check if kind is supported
    if SUPPORTED.contains(&config.kind.as_str()) {
        match verify_url(&config) {
            Ok(_) => return Ok(config),
            Err(e) => return Err(e),
        }
    } else {
        // or error
        error!("{} kind of config not supported, moving on", &config.kind);
        Err(ConfigError::NotSupportedError)
    }
}

// Verify url will ensure that a url is valid
fn verify_url(conf: &Config) -> Result<(), ConfigError> {
    match Url::parse(&conf.url.as_str()) {
        Ok(_) => return Ok(()),
        Err(e) => {
            error!("url {} is not valid", &conf.url.as_str());
            return Err(ConfigError::from(e));
        }
    };
}

// Functions - public
// Create a default test config (used in tests)
#[allow(dead_code)]
pub fn create_test_config() -> Config {
    return Config::new(
        "rage.love".to_string(),
        "https://rage.love".to_string(),
        "mastodon".to_string(),
        None,
    );
}

// Get all configs from config.d
pub fn get_configs_files(dir: &str) -> Result<Vec<Config>, ConfigError> {
    // walk in dir and get files
    let paths = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    info!("Start reading all files in {} directory", dir);

    // loop over all paths and read each entry or associated error
    let (res, errs): (Vec<_>, Vec<_>) = paths
        .into_iter()
        .map(|c| read_config_file(&c))
        .partition(Result::is_ok);

    // errors are wrapped into results, to unwrap errors
    let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();

    // same for configs
    let configs: Vec<_> = res.into_iter().map(Result::unwrap).collect();

    // print errors
    for err in errors {
        error!("{:?}", err)
    }

    // If some configs found return all of them
    if configs.len() > 0 {
        return Ok(configs);
    } else {
        // If not return an error
        return Err(ConfigError::NoConfigError);
    }
}

// Tests
// Tester c'est douter
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_verify_url() {
        // prepate
        let mut config = create_test_config();

        config.url = "ohno".to_string();

        // assert
        match verify_url(&config) {
            Ok(_) => assert!(true),
            Err(_) => panic!("verify_url test should failed... :shrug:"),
        }
    }
}
