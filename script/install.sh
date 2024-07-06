#!/usr/bin/env bash

git clone https://github.com/Young-TW/yu.git
cd yu || exit
cargo build --release
sudo cp target/release/yu /usr/local/bin/yu
