# Fediwatcher

[![Build Status](https://drone.github.papey.fr/api/badges/papey/fediwatcher/status.svg)](https://drone.github.papey.fr/papey/fediwatcher)

Fediwatcher is a [rust](https://www.rust-lang.org/) app used to fetch various
metrics from [fediverse instances](https://fediverse.party) and push all the
results into an InfluxDB server.

Fediwatcher currently supports :

- Mastodon : to get metrics from a Mastodon instance
- Mastodon User : to get metrics from a specific user on a Mastodon instance
- Pleroma : to get metrics from a Pleroma instance
- Pleroma User : to get metrics from a specific user on a Pleroma instance

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [InfluxDB](https://www.influxdata.com)
- [Grafana](https://grafana.com) (Optional)
- [Docker](https://www.docker.com/) (Optional)

### Installing

#### Setup InfluxDB & Grafana

docker-compose is the easiest solution to get stuff up and running

```sh
docker-compose up -d
```

Then add the InfluxDB datasource inside Grafana (see related Grafana documentation)

#### Get Fediwatcher

##### From source

Clone this repo and run

```sh
cargo build --release
```

##### From Docker Hub

See [papey/fediwatcher](https://hub.docker.com/r/papey/fediwatcher) on Docker Hub

### Usage

```sh
./target/debug/fediwatcher --help
```

Fediwatcher uses config files in .toml format, see `conf.d` directory inside
the `tests` directory for real life examples.

To specify a custom InfluxDB server, Fediwatcher use environment variables, eg :

- INFLUXDB_DATABASE=fediwatcher
- INFLUXDB_PASSWORD=f3d1w4tch3r
- INFLUXDB_USER=fediwatcher
- INFLUXDB_HOST=[http://localhost:8086](http://localhost:8086)

#### Notes

In order to refresh data, you need to run fediwatcher periodicaly using
systemd timers or any other method of your choice.

## Running the tests

```sh
CONFD=./tests/conf.d cargo test
```

## Continous Integration

See [drone.github.papey.fr/papey/fediwatcher](https://drone.github.papey.fr/papey/fediwatcher)

## Built With

- [InfluxDB-rust](https://github.com/Empty2k12/influxdb-rust) - InfluxDB rust library
- [Clap](https://github.com/clap-rs/clap) - Command line parser

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

- **Wilfried OLLIVIER** - _Main author_ - [Papey](https://github.com/papey)

## License

[LICENSE](LICENSE) file for details

## Public Instance

Want to see Fediwatcher in action ? No problem ! There is a [public instance](https://metrics.papey.fr) !

Want metrics for you instance or you as a user ? Just [contact me](https://papey.fr/#contact) !

Finaly, all dashboards on the public instance can be downloaded freely using the `share` button on Grafana UI ! Enjoy !

## Acknowledgments

- Inspired by [fediverse.network](https://fediverse.network) made by [fediverse](https://github.com/fediverse)
- This is mainly a rust learning project, so huge thanks to all the rusteceans from Mastodon and the [Rust Lang Discord
  Server](https://discordapp.com/invite/rust-lang) for all the help !
