# Rust spike of the forms-api-prototype

## prerequisites

Rust and either a local Postgres, or Docker. 

### Rust

Install the rust toolchain from https://sh.rustup.rs with:

``` shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Docker

If you don't have Docker, install it by following the instructions here: https://docs.docker.com/get-docker/

### Postgres

Though Docker is a cleaner way to run Postgres, any local Postgres db will work for testing.
The installation and configuration of which is left as an exercise for the user. Look in `docker-compose.yaml` for the credentials and db we expect.

## Running the server


Please make sure your postgres db is up before starting the server, to do this with the included docker config is simply:

``` shell
docker-compose up
```

Then when your postgres is up, however you have done that, start the api server:

```shell
cargo run
```


