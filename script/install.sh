#!/usr/bin/env bash

# Check Git installed

if ! command -v git &> /dev/null; then
    printf "Git is not installed. Do you want to install it? (y/n): "
    read -r response
    if [[ $response != "y" ]]; then
        echo "Please install Git"
        exit 1
    fi
    # check package manager
    if command -v apt &> /dev/null; then
        sudo apt install git
    elif command -v pacman &> /dev/null; then
        sudo pacman -S git
    elif command -v dnf &> /dev/null; then
        sudo dnf install git
    elif command -v brew &> /dev/null; then
        brew install git
    else
        echo "Can't find package manager, please install Git by yourself"
        exit 1
    fi
fi

# Check Rust and Cargo installed

if ! command -v rustc &> /dev/null; then
    printf "Rust is not installed. Do you want to install it? (y/n): "
    read -r response
    if [[ $response != "y" ]]; then
        echo "Please install Rust"
        exit 1
    fi
    # check package manager
    if command -v apt &> /dev/null; then
        sudo apt install rustc
    elif command -v pacman &> /dev/null; then
        sudo pacman -S rust
    elif command -v dnf &> /dev/null; then
        sudo dnf install rust
    elif command -v brew &> /dev/null; then
        brew install rust
    else
        echo "Can't find package manager, please install Rust by yourself"
        exit 1
    fi
fi

# Install yu

git clone https://github.com/Young-TW/yu.git
cd yu || exit
cargo build --release
sudo cp target/release/yu /usr/local/bin/yu
