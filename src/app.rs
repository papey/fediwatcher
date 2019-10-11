use crate::config;
use crate::get;
use crate::influx;
use influent::client::ClientError as InfluentError;

// AppError
// Define AppError
#[derive(Debug)]
pub enum AppError {
    GetError(get::GetError),
    Str(String),
    InfluxError(InfluentError),
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
impl From<InfluentError> for AppError {
    fn from(err: InfluentError) -> AppError {
        AppError::InfluxError(err)
    }
}

pub fn run(matches: clap::ArgMatches) -> Result<(), AppError> {
    // get configs info by walking inside conf.d directory
    let configs = match config::get_configs_files(matches.value_of("conf.d").unwrap()) {
        Some(configs) => configs,
        None => panic!(
            "No config file found in {} directory",
            matches.value_of("conf.d").unwrap()
        ),
    };

    // ensure conn to influx
    let client = influx::push::create_influx_client(
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
            Err(_) => warn!("Error getting data for config {}", conf.name),
        }
    }

    Ok(())
}
