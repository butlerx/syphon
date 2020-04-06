# Welcome to syphon 👋

![Version](https://img.shields.io/crates/v/syphon?style=flat-square)
[![License: Apache License 2.0](https://img.shields.io/badge/License-Apache%20License%202.0-yellow.svg)](./LICENSE)
[![Twitter: cianbutlerx](https://img.shields.io/twitter/follow/cianbutlerx.svg?style=social)](https://twitter.com/cianbutlerx)

> Versatile metrics processor, proxy and forwarder

## Install

```bash
git clone github.com/butlerx/syphon
cd syphon
cargo Install
```

## Usage

```bash
$ syphon -h
syphon 0.1.0
butlerx <butlerx@notthe.cloud>
Versatile metrics processor, proxy and forwarder
syphon is designed to accept and route metrics traffic.
Metrics can be received from socket, snooped from live traffic or read from file or grpc.
Metrics can be exportered via file, grpc or udp/tcp

USAGE:
    syphon [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -p, --print      Print default config
    -q               Silence all output
    -V, --version    Prints version information
    -v               Increase message verbosity

OPTIONS:
    -c, --config <CONFIG>    Config file to load (default: configs/config.toml)
```

## Configuration

```toml
[metric]
# Endpoint for store internal carbon metrics.
# Valid values: "" or "local", "tcp://host:port", "udp://host:port"
endpoint = "local"
# Interval of storing internal metrics. Like CARBON_METRIC_INTERVAL
interval = "1m0s"

[[uploader.udp]]
enabled = false
host = "127.0.0.1"
port = 2004

[[uploader.tcp]]
enabled = false
host = "127.0.0.1"
port = 2004

[[uploader.file]]
enabled = true
path = "metrics_received.txt"
# RegEx pattern to use to Determine if metric should be sent
# Supports  full regex from https://docs.rs/regex/1.3.6/regex/
pattern="metric.path*"

# Designed for use with carbon-clickhouse
# https://github.com/lomik/carbon-clickhouse/blob/master/grpc/carbon.proto
[[uploader.grpc]]
enabled = false
host = "127.0.0.1"
port = 2005

[file]
enabled = true
path = "metrics.txt"

[udp]
enabled = true
listen = "127.0.0.1:2003"
# Setting mode to promiscuous sets the interface to promiscuously listen
# Allows to see traffic on a port already in use
# mode = "promiscuous"

[tcp]
enabled = true
listen = "127.0.0.1:2003"

[prometheus]
enabled = true
listen = "127.0.0.1:2006"
```

## Run tests

```sh
cargo check
cargo clippy
cargo test
```

## Author

👤 **Cian Butler**

- Website: [cianbutler.ie](https://cianbutler.ie)
- Twitter: [@cianbutlerx](https://twitter.com/cianbutlerx)
- Github: [@butlerx](https://github.com/butlerx)
- LinkedIn: [@butlerx](https://linkedin.com/in/butlerx)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!

Feel free to check [issues page](https://github.com/butlerx/syphon/issues).

## Show your support

Give a ⭐️ if this project helped you!

## 📝 License

Copyright © 2020 [Cian Butler](https://github.com/butlerx).

This project is [Apache License 2.0](./LICENSE) licensed.
