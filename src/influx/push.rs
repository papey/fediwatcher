// Mod push - used to push data to remote inluxdb
// Uses
use crate::influx::translate::{DataField, Measurement};
use influent::client::{http::HttpClient, Client, ClientWriteResult, Credentials};
use influent::create_client as influx_client;
use influent::measurement::Measurement as InfluentMeasurement;
use influent::measurement::Value;

// Functions - public
// create_influx_client creates an influx db client
pub fn create_influx_client<'a>(
    username: &'a str,
    password: &'a str,
    database: &'a str,
    host: &'a str,
) -> HttpClient<'a> {
    let creds = Credentials {
        username,
        password,
        database,
    };

    let mut hosts = Vec::new();
    hosts.push(host);

    influx_client(creds, hosts)
}

// push_measurement push measurement paramater to output configured in client
pub fn push_measurement(client: &HttpClient, measurement: Measurement) -> ClientWriteResult {
    let mut data = InfluentMeasurement::new(&measurement.key);

    for (key, value) in measurement.fields.iter() {
        match value {
            DataField::Int(value) => {
                data.add_field(key, Value::Integer(*value));
            }
            DataField::Str(value) => {
                data.add_field(key, Value::String(value.as_str()));
            }
        }
    }

    for (key, value) in measurement.tags.into_iter() {
        data.add_tag(key, value);
    }

    client.write_one(data, None)
}
