#!/bin/sh
cargo build --release
sudo cp ./target/release/hypofetch /usr/bin
