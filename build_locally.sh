#!/bin/bash

# Builds using cargo, leaving the executable in the current directory.
cargo build --release
cp ./target/release/vc .