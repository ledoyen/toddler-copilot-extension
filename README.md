# Toddler copilot-extension
[![🏗️ Build and test](https://github.com/ledoyen/toddler-copilot-extension/actions/workflows/ci.yml/badge.svg)](https://github.com/ledoyen/toddler-copilot-extension/actions/workflows/ci.yml)

Sandbox for experimenting with GitHub Copilot extension mechanism.

Primary instance is available at https://toddler-copilot-extension-xgtm.shuttle.app/

## Use Just

Install just : https://github.com/casey/just?tab=readme-ov-file#installation

Then enter `just` to see the list of available recipes.

## Configuration

The app in itself have the following configuration parameters:

| Name                                |                                     | Description                                                         | Example                               |
|-------------------------------------|-------------------------------------|---------------------------------------------------------------------|---------------------------------------|
| BASE_URL                            | Required                            | Used to compute callback urls (such as the one for oauth2 web flow) | http://localhost:8000                 |
| GITHUB_APP_CLIENT_ID                | Required                            | Client ID of the GitHub app                                         | Abd.YTGB4541hj                        |
| GITHUB_APP_CLIENT_SECRET            | Required                            | Client Secret of the GitHub app                                     | ad45f12ccb5687                        |

## Run it locally

This is a Rust project, using Shuttle as IFC environment.

* Install Rust and its ecosystem if you have not already
    * `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` cf https://rustup.rs/
* Install Just
    * `cargo install just` cf https://just.systems/man/en/chapter_4.html
* Install Shuttle
    * `cargo install cargo-shuttle` cf https://docs.shuttle.rs/getting-started/installation

* Clone and build
    * `git clone git@github.com:korekto/korekto-shuttle.git`
    * `cd korekto-shuttle`

* Create and fill the `Secrets.toml` file with expected [configuration](#Configuration) parameters,
  see [docs](https://docs.shuttle.rs/resources/shuttle-secrets)

* Start the app
    * `clear && just run`
