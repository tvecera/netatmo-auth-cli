# Netatmo Auth CLI

Netatmo is removing username and password authorization at the end of September 2022. Logging in and allowing access to the API requires interaction with the user. It's a problem for integration, e.g. via Node-RED.

This simple command line client generates the URL for authentication and exposes the endpoint to the Netatmo OAUTH provider.

## Build

You can use pre-build binaries (I tested only macOS binary), or you can compile the application yourself. You will need RUST installed on your computer.

Building the application is then straightforward:

`$ cargo build --release`

## Run
`$ ./netatmo-auth-cli --client CLIENT_ID --secret CLIENT_SECRET`

Follow the instructions given by the application.

**It's the alpha version and tested only on macOS.** 