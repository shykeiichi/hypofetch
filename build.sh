#!/bin/sh

cargo build --release
cp ./target/release/hypofetch /usr/bin
cp ./hypo ~/.config/ -r
