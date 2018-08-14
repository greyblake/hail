# Hail

HTTP load testing tool powered by Rust and Tokio.

## Installation

There are 2 options to install:

* Download a static build from [releases](https://github.com/greyblake/hail/releases)
* Install with cargo: `cargo install hail`

## Usage


The following command sends 100 http requests to `http://example.com`:

```
hail http://example.com --concurrent 10 --requests 100
```

and prints result:

```
Requests sent: 100
Total time: 2.413s
Avg response time: 0.241s
OK rate: 100.00%
Error rate: 0.00%
```
