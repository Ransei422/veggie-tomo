#!/bin/bash

# Set log level
export RUST_LOG=info

# Run it baby (and ignore logs because this will create infinity loop in cargo watch)
cargo watch -i logs/ -x 'run --release'

# For debug mode run ↓
# cargo watch -i logs/ -x run