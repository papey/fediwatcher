// Main
// Extern crates
#[macro_use]
extern crate log;

// mods
mod config;
mod get;
mod influx;

// Uses
use clap::{App, Arg};
use influent::client::ClientError as InfluentError;

// AppError
// Define AppError
#[derive(Debug)]
enum AppError {
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

// Entry point
fn main() -> Result<(), AppError> {
    env_logger::init();

    // Setup command line args and options
    let matches = App::new("Fediwatcher")
        // basic stuff
        .name("Fediwatcher")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("A small rust app to fetch various metrics from the fediverse")
        .long_about(
            r#"Fediwatcher is a rust app used to fetch various metrics from fediverse instances
and push all the results into an InfluxDB server"#,
        )
        // log level
        .arg(
            Arg::with_name("loglevel")
                .help("Set loglevel using RUST_LOG environment variable")
                .env("RUST_LOG")
                .default_value("ERROR"),
        )
        // conf.d containing all the conf.toml config file
        .arg(
            Arg::with_name("conf.d")
                .short("c")
                .long("conf.d")
                .env("CONFD")
                .help("Path to directory containing config.toml files")
                .default_value("/etc/fediwatcher/conf.d"),
        )
        // influxdb
        // database
        .arg(
            Arg::with_name("influx_database")
                .env("INFLUXDB_DATABASE")
                .default_value("fediwatcher")
                .help("Name of the InfluxDB database"),
        )
        // password
        .arg(
            Arg::with_name("influx_password")
                .env("INFLUXDB_PASSWORD")
                .default_value("f3d1w4tch3r")
                .help("Password used by InfluxDB user"),
        )
        // user
        .arg(
            Arg::with_name("influx_user")
                .env("INFLUXDB_USER")
                .default_value("fediwatcher")
                .help("Name of the InfluxDB user"),
        )
        // host
        .arg(
            Arg::with_name("influx_host")
                .env("INFLUXDB_HOST")
                .default_value("http://localhost:8086")
                .help("URL of InfluxDB endpoint"),
        )
        // get all the matches and ! good to go !
        .get_matches();

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

        // get data
        let data = get::get_data(&conf)?;

        // translate data
        let measurement = influx::translate::new_from(&data, &conf)?;

        // push data to influxdb
        influx::push::push_measurement(&client, measurement)?;
    }

    Ok(())
}
