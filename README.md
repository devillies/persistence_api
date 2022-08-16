# Requirement

- use local postgres or docker postgres
- libpq
- diesel_cli
- rust

## docker postgres

`docker run -d -p 5432:5432 -e POSTGRES_USER=<username> -e POSTGRES_PASSWORD=<password> postgres:14-alpine`

> note: for the postgres image you can use the latest postgres by
> changing the ` postgres:alpine-14` to `postgres`

> in this project i use ` postgres:14-alpine` because it is known to be lighter than other images

## installing diesel_cli

`cargo install diesel_cli --no-default-features --features postgres`

## diesel setup

run `diesel setup` in the terminal

## Running the project

to run the project , just run this command in the terminal:
`cargo run .`
