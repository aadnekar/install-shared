# Install Shared

This is a utility script in order to easily generate types, pack and install pc-nrfconnect-shared.

## Requirements

- You need to set an environment variable `SharedWorkspace=<location of pc-nrfconnect-shared>`
- You need npm installed
- To build from source, you need cargo

## Build

Build the script for release

```bash
cargo build --release
```

## Usage

Recommend to first put the executable somewhere in your PATH, e.g. on Ubuntu you can move it to `/$HOME/.local/bin`.

```bash
cp target/release/install-shared ~/.local/bin/

# Run the script inside one of the pc-nrfconnect apps:
install-shared
```
