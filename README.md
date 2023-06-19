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

You'll need to insert some Secrets into a `Secrets.toml` file (see `Secrets.toml.example` for details).

Before you run the backend folder you'll probably want to compile the frontend assets which you can do by simply going to the frontend folder and using `npm run build`.

When you're ready, simply run `cargo start` from the project root or `cargo shuttle run` from the backend folder!

### Features

* Discord bot 

  * Has commands for linking to module documentation

  * Has command for elevating Discord threads to Github issues (locks Discord thread, opens Github issue)

    * This bot supports using Github Apps.

  * Has commands for (un)locking threads and setting the severity of an issue.

* Visual dashboard

  * Pull statistics for ticketing (who solved the most tickets, most common ticket category, etc)

  * General issues

### Todo

* Github OAuth for dashboard

* Search filter for issues

* Github web hooks

### Dependencies

| Dependency        | Reason for Dependency                                             |
|-------------------|-------------------------------------------------------------------|
| anyhow            | Easy errors. Might change this at some point.                     |
| axum              | An easy to use framework with familiar syntax.                    |
| jsonwebtoken      | Making JWT to be able to do app auth (for GitHub app)             |
| octocrab          | Interact with GitHub API easily                                   |
| poise             | Discord bot framework (built on Serenity)                         |
| serde             | (de)Serialization of structs for JSON responses                   |
| shuttle-runtime   | Shuttle dependency                                                |
| shuttle-secrets   | Environmental variables on Shuttle                                |
| shuttle-poise     | Allows the Shuttle runtime to use Poise                           |
| shuttle-shared-db | Provisioned database via Shuttle (uses postgres with sqlx pool)   |
| sqlx              | Raw SQL is faster (also: allows easy interfacing with shuttle DB) |
| tokio             | Required for async and is a Shuttle dependency                    |
| tracing           | Logging/tracing for errors                                        |

