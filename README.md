<p align="center">
    <img src="https://github.com/preiter93/stellarpc/blob/main/img/logo-light.png?raw=true#gh-light-mode-only" width="600"/>
    <img src="https://github.com/preiter93/stellarpc/blob/main/img/logo-dark.png?raw=true#gh-dark-mode-only" width="600"/>
</p>

## What is StellaRPC?

StellaRPC is a gRPC client that runs in the terminal! Simply put your proto files in a config and be ready to make gRPC request

## Demo

![](img/screen-1.png)

![](img/screen-2.png)

## Features

#### Maintain a Request Log
You now have the option to preserve your most recent request (S) and access a history of all saved requests (H), including the default template. This feature eliminates the need to ponder over the contents of your request.

#### Copy as grpcurl
stellaRPC offers the ability to copy the request data as a grpcurl command, streamlining collaboration with your peers. Simply navigate to the request page and press Ctrl+Y.

## Roadmap

- [x] List Services & Methods
- [x] Request can be edited
- [x] TLS support
- [x] Unary gRPC client calls
- [ ] Streaming gRPC client calls
- [x] Metadata specification
- [x] Server address specification
- [x] Request History
- [ ] Extended message description
- [x] Defaults of repeated/nested fields
- [x] Yank/Paste from clipboard
- [x] Yank request as grpcurl command
