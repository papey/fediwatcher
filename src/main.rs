// Main
// Extern crates
#[macro_use]
extern crate log;

// mods
mod app;
mod config;
mod get;
mod influx;

// Uses
use clap::{App, Arg};
use std::process;

// Entry point
fn main() {
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

    // Check if the returned value is an error
    if let Err(e) = app::run(matches) {
        // print error to stderr
        eprintln!("Application error: {:?}", e);

        // exit with an error code
        process::exit(1);
    }
}
