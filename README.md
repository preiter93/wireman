<p align="center">
    <img src="https://github.com/preiter93/stellarpc/blob/main/resources/logo-light.png?raw=true#gh-light-mode-only" width="600"/>
    <img src="https://github.com/preiter93/stellarpc/blob/main/resources/logo-dark.png?raw=true#gh-dark-mode-only" width="600"/>
</p>

## What is StellaRPC?

StellaRPC is a terminal-based gRPC client. All you need to do is place your protobuf files into a config file, and you're all set to start making gRPC requests.


## Configuration Setup

The program looks for a configuration file specified by the environment variable `STELLARPC_CONFIG`. The configuration file should be in JSON format and resembles the following:
```json
{
    "address": "http://localhost:50051",
    "history": "$HOME/.config/stellarpc/history",
    "includes": [
        "$HOME/your-project/protos"
    ],
    "files": [
        "grpc_simple/greeter.proto",
        "grpc_simple/timekeeper.proto"
    ]
}
```
Within this configuration, you can store a list of include directories, proto files and a default address host. Please note that these configuration options may expand in the future. In the future there will be a feature in the app that will allow you to manually edit this config.

## Demo

![](resources/demo.gif)

## Features

#### Maintain a Request History
You can save up to five histories histories per request and switch between them by typing numbers 1 to 5. Save your preferred history by typing S, and restore the default message by typing ctrl+d.

#### Copy as grpcurl
stellaRPC offers the ability to copy the request data as a grpcurl command, streamlining collaboration with your peers. Simply navigate to the request page and press ctrl+y.

#### Format request
Press ctrl+f to format the json of the request message.

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

Planned
- [ ] Set metadata
- [ ] Edit config file in app
- [ ] Provide installation help

Maybe
- [ ] Streaming gRPC
