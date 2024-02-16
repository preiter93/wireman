<p align="center">
    <img src="https://github.com/preiter93/wireman/blob/main/resources/logo-light.png?raw=true#gh-light-mode-only" width="700"/>
    <img src="https://github.com/preiter93/wireman/blob/main/resources/logo-dark.png?raw=true#gh-dark-mode-only" width="700"/>
</p>

# What is WireMan?

WireMan is a terminal-based gRPC client with a user-friendly interface. It reads `.proto` files from a config file and keeps a history of requests.
WireMan is an ideal choice for developers testing gRPC endpoints directly from the terminal.

# Getting Started with WireMan

This guide will walk you through the steps to set up and run WireMan.

## Prerequisites

- Rust: [Minimum required Rust version is 1.70] ([Installation Guide](https://www.rust-lang.org/tools/install))

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/preiter93/wireman.git
    ```

2. Build in release mode:

    ```bash
    cd wireman
    cargo build --release
    ```

3. Copy the binary to your PATH, e.g.:

    ```bash
    cp target/release/wireman /usr/local/bin
    ```

## Configuration

1. Set the `WIREMAN_CONFIG_DIR` environment variable to specify the directory where your configuration file will be located:

    ```bash
    export WIREMAN_CONFIG_DIR=~/.config/wireman
    ```

2. Create the directory specified in `WIREMAN_CONFIG_DIR`:

    ```bash
    mkdir -p ~/.config/wireman
    ```

3. Create a `config.toml` file in the specified directory. Here's an example configuration:

    ```toml
    includes = [
        "$HOME/your-project"
    ]
    files = [
        "a-proto-file-in-your-project.proto"
    ]
    default_address = "http://localhost:50051"
    history_dir = "$HOME/.config/wireman/history"
    ```

    Replace with the appropriate values for your project.

## Usage

1. After adding the protos in the config, the first page of Wireman will list your grpc services and their methods. You can navigate with up/down or j/k, select with enter, then click tab.

2. The second page is the message screen. Edit your request message, you can format it with Ctrl+f. Then click tab. The navigation of the editor is vim-inspired: use h, j, k, l for motion, go into insert mode with i, and escape insert mode with Esc. For more details, refer to [edtui](https://github.com/preiter93/edtui).
 
4. The third page is the config screen where you can edit the address or metadata. Wireman allows for using bash scripts to get your tokens. Place the name of your script in the bearer token field, which must be in your PATH, between `$()`, e.g. `$(getToken.sh)`. Go back with Shift+Tab or click tab twice.

5. Now you can make the request by clicking Enter. You can save it with Ctrl+s. Saved requests are stored in spots 1 to 5. Switch to a spot by clicking the respective number. You can restore the default request with Ctrl+d, which deletes the history.

6. If you want to get the current request as a gRPCurl command, click Ctrl+y on the request tab, and it's copied to your clipboard.

## Demo

![](resources/demo.gif)

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
- [ ] Custom themes
- [ ] Command line help

Maybe
- [ ] Streaming gRPC
