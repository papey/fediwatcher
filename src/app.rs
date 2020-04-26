use crate::config;
use crate::get;
use crate::influx;
use crate::influx::translate;
use influxdb::Error as InfluxError;

// AppError
// Define AppError
#[derive(Debug)]
pub enum AppError {
    GetError(get::GetError),
    Str(String),
    InfluxError(InfluxError),
    ConfigError(config::ConfigError),
    TranslateError(translate::TranslateError),
}

// GetError
impl From<get::GetError> for AppError {
    fn from(err: get::GetError) -> AppError {
        AppError::GetError(err)
    }
}

// Str
impl From<String> for AppError {
    fn from(err: String) -> AppError {
        AppError::Str(err)
    }
}

// InfluxError
impl From<InfluxError> for AppError {
    fn from(err: InfluxError) -> AppError {
        AppError::InfluxError(err)
    }
}

// InfluxError
impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> AppError {
        AppError::ConfigError(err)
    }
}

// TranslateError
impl From<translate::TranslateError> for AppError {
    fn from(err: translate::TranslateError) -> AppError {
        AppError::TranslateError(err)
    }
}

pub fn run(matches: clap::ArgMatches) -> Result<(), AppError> {
    // get configs info by walking inside conf.d directory
    let configs = config::get_configs_files(matches.value_of("conf.d").unwrap())?;

    // ensure conn to influx
    let client = influx::push::create_influx_client(
        // unwraping is ok here since defaults value are set
        matches.value_of("influx_user").unwrap(),
        matches.value_of("influx_password").unwrap(),
        matches.value_of("influx_database").unwrap(),
        matches.value_of("influx_host").unwrap(),
    );

    for conf in configs {
        // analysing conf
        debug!("Analysing conf {} of kind {}", &conf.name, &conf.kind);

        match get::get_data(&conf) {
            Ok(data) => {
                // translate data
                let measurement = influx::translate::new_from(&data, &conf)?;
                // push data to influxdb
                influx::push::push_measurement(&client, measurement)?;
            }
            Err(e) => {
                error!("{:?}", e);
                warn!("Error getting data for config {}", conf.name);
            }
        }
    }

    Ok(())
}
