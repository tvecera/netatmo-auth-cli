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

## Listen on specific IPv4 or IPv6 address (v0.2.0)

You can specify which IP address the application will listen on. By default, the application listens on 127.0.0.1. 

`$ ./netatmo-auth-cli --client CLIENT_ID --secret CLIENT_SECRET --host 10.0.0.1`

**It's the alpha version and tested only on macOS.** 