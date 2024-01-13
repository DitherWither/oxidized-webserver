# oxidized-webserver

Web server in rust with zero dependencies/crates

TODO: Add a empty application that serves static files

It uses TCP/IP directly

## Building

To build a podman image, run
```bash
scripts/build.sh
```
and it should generate an image called `oxidized-webserver:latest`.

The image currently serves a hello world application

It should also work with docker

## Build/Run locally

Just use `cargo run` to run and `cargo build` to build

## Configuration

It runs at port 8080 by default, can be configured by the `PORT` environment variable
