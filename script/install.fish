#!/usr/bin/env fish

# Check Git installed
if not type -q git
    printf "Git is not installed. Do you want to install it? (y/n): "
    read response
    if test "$response" != "y"
        echo "Please install Git"
        exit 1
    end
    # check package manager
    if type -q apt
        sudo apt install git
    else if type -q pacman
        sudo pacman -S git
    else if type -q dnf
        sudo dnf install git
    else if type -q brew
        brew install git
    else
        echo "Can't find package manager, please install Git by yourself"
        exit 1
    end
end

# Check Rust and Cargo installed
if not type -q rustc
    printf "Rust is not installed. Do you want to install it? (y/n): "
    read response
    if test "$response" != "y"
        echo "Please install Rust"
        exit 1
    end
    if type -q apt
        sudo apt install rustc
    else if type -q pacman
        sudo pacman -S rust
    else if type -q dnf
        sudo dnf install rust
    else if type -q brew
        brew install rust
    else
        echo "Can't find package manager, please install Rust by yourself"
        exit 1
    end
end

# Install yu
git clone https://github.com/Young-TW/yu.git
cd yu
cargo build --release
sudo cp target/release/yu /usr/local/bin/yu
