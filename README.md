### Shuttlebot

This is a web service that facilitates support ticket management on Discord forum help channels.

Built with love in Rust.

### How to Use

Looking to run this for yourself? You'll need `cargo-shuttle`, which you can install with the following:

```sh
// run this if you don't use cargo-binstall
cargo install cargo-shuttle
// run this if you do use cargo-binstall
cargo binstall cargo-shuttle
```

You'll also need Docker to get a local database provisioned to yourself, which you can find instructions on how to install [here](https://docs.docker.com/get-docker/).

Clone the repository then simply run `cargo shuttle run`. 

### Features

* Discord bot 

  * Has commands for linking to module documentation

  * Has command for elevating Discord threads to Github issues (locks Discord thread, opens Github issue)

### Todo

* Dashboard to pull statistics for ticketing (who solved the most tickets, most common ticket category, etc)


### Dependencies

| Dependency        | Reason for Dependency                                             |
|-------------------|-------------------------------------------------------------------|
| anyhow            | Easy errors. Might change this at some point.                     |
| octocrab          | Interact with GitHub API easily                                   |
| poise             | Discord bot framework (built on Serenity)                         |
| shuttle-runtime   | Shuttle dependency                                                |
| shuttle-secrets   | Environmental variables on Shuttle                                |
| shuttle-poise     | Allows the Shuttle runtime to use Poise                           |
| shuttle-shared-db | Provisioned database via Shuttle (uses postgres with sqlx pool)   |
| sqlx              | Raw SQL is faster (also: allows easy interfacing with shuttle DB) |
| tokio             | Required for async and is a Shuttle dependency                    |
| tracing           | Logging/tracing for errors                                        |

