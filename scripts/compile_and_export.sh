#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

# Compile the project
cargo build --release

# Export the project
cp target/release/wedp ./tests/wedp