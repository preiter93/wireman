<p align="center">
    <img src="https://github.com/preiter93/wireman/blob/main/resources/logo-light.png?raw=true#gh-light-mode-only" width="700"/>
    <img src="https://github.com/preiter93/wireman/blob/main/resources/logo-dark.png?raw=true#gh-dark-mode-only" width="700"/>
</p>

## What is WireMan?

WireMan is a terminal-based gRPC client with a user-friendly interface. It reads `.proto` files from a config file and keeps a history of requests.
WireMan is an ideal choice for developers testing gRPC endpoints directly from the terminal.

## Configuration Setup

At startup, WireMan searches for a configuration file that is specified in the `WIREMAN_CONFIG` environment variable:
```
export WIREMAN_CONFIG="$HOME/.config/wireman/config.json"
```
The configuration file looks like this:
```json
{
    "address": "http://localhost:50051",
    "history": "$HOME/.config/wireman/history",
    "includes": [
        "$HOME/your-project/protos"
    ],
    "files": [
        "grpc_simple/greeter.proto",
        "grpc_simple/timekeeper.proto"
    ]
}
```
With this configuration, the default host address is `localhost:50051`. The request history is saved in the directory `$HOME/.config/wireman/history`. `Includes` defines a list of directories in which to search for proto files and `files` specifies all `.proto` files to be loaded into WireMan.

## Demo

![](resources/demo.gif)

## Features

#### Maintain a Request History
You can save up to five histories per request and switch between them by typing numbers 1 to 5. Save your preferred history by typing ctrl+s, and delete a history with ctrl+d.

The directory in which the history is saved is defined in the config under "history". If this field is left empty, WireMan does not save a request history.

#### Copy as grpcurl
WireMan offers the option of copying the request data as a raw `grpcurl` command, which simplifies collaboration with your colleagues. Navigate to the request page and press ctrl+y.

#### Format request
On the request page, press ctrl+f. This formats the request message as pretty json (if possible).

## Roadmap

- [x] Unary gRPC
- [x] Set host address
- [x] Set authentication headers
- [x] Request History
- [x] Defaults of repeated/nested fields
- [x] Yank/Paste from clipboard
- [x] Yank request as grpcurl command
- [x] Vim like editor feeling
- [x] Show loading indicator
- [x] Metadata headers

Planned
- [ ] Edit config file in app
- [ ] Provide installation help

Maybe
- [ ] Streaming gRPC
