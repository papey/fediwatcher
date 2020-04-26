// Mod push - used to push data to remote inluxdb
// Uses
use crate::influx::translate::{DataField, Measurement};
use influxdb::InfluxDbWriteable;
use influxdb::{Client, Error, Timestamp};
use tokio::runtime::Runtime;

// Functions - public
// create_influx_client creates an influx db client
pub fn create_influx_client<'a>(
    username: &'a str,
    password: &'a str,
    database: &'a str,
    host: &'a str,
) -> Client {
    influxdb::Client::new(host, database).with_auth(username, password)
}

// push_measurement push measurement paramater to output configured in client
pub fn push_measurement(client: &Client, measurement: Measurement) -> Result<String, Error> {
    // init query
    let mut query = Timestamp::Now.into_query(&measurement.key);

    // for each field add to query
    for (key, value) in measurement.fields.iter() {
        match value {
            // If int
            DataField::Int(value) => {
                query = query.add_field(key, value);
            }
            // If Str a clone is needed
            DataField::Str(value) => {
                query = query.add_field(key, value.clone());
            }
        }
    }

    // for each tag add to query
    for (key, value) in measurement.tags.into_iter() {
        query = query.add_tag(key, value);
    }

    // write query, handle future simply with block on
    Runtime::new().unwrap().block_on(client.query(&query))
}
