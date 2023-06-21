### Shuttlebot

This is a web service that facilitates support ticket management on Discord forum help channels.

Built with love in Rust.

### Pre-reqs to Use

Looking to run this for yourself? You'll need `cargo-shuttle`, which you can install with the following:

```sh
// run this if you don't use cargo-binstall
cargo install cargo-shuttle
// run this if you do use cargo-binstall
cargo binstall cargo-shuttle
```

You'll also need Docker to get a local database provisioned to yourself, which you can find instructions on how to install [here](https://docs.docker.com/get-docker/).

You'll need a [Github App](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app) and a [Github Oauth app](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/creating-an-oauth-app).

You'll also need a Discord app with the following bot permissions:



### How to Use

Clone the repo then `cd` to it: 

```sh
git clone https://github.com/joshua-mo-143/shuttlebot.git
cd shuttlebot
```

You'll need to insert some Secrets into a `Secrets.toml` file (see `Secrets.toml.example` for details). Details of each secret can be found below:

| Secret name                  | Usage                                                                                                                                                         |
|------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|
| DISCORD_TOKEN                | Used to be able to set up a Discord bot (REQUIRED)                                                                                                            |
| GITHUB_PERSONAL_TOKEN        | Used to authenticate Octocrab so you can interact with the Github API.  This is set in the app to be the fallback if the PEM key file contents doesn't exist. |
| GITHUB_APP_ID                | ID for your Github app (see [this](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app)).                  |
| DISCORD_SERVER_STAFF_ROLE_ID | The ID of the required role for users to execute certain commands (currently on every command except the Docs command).                                       |
| DISCORD_SERVER_ID            | The Guild ID of a Discord channel (basically, the server ID).                                                                                                 |
| GITHUB_OAUTH_ID              | The ID of your Github Oauth app.                                                                                                                              |
| GITHUB_OAUTH_SECRET          | The secret for your Github Oauth app.                                                                                                                         |
| GITHUB_APP_PRIVATE_KEY       | The contents of the RSA key from the PEM file you get when creating a secret for a Github app.                                                                |

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

  * Github Oauth

### Todo

* Search filter for issues

* Github web hooks

* Background task to delete expired sessions from shuttle-persist

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

