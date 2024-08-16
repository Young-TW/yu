# yu - a package manager wrapper for all unix-like systems

`yu` allows users to use the same command to install, uninstall, and upgrade packages on all unix-like systems.

[![CodeFactor](https://www.codefactor.io/repository/github/young-tw/yu/badge)](https://www.codefactor.io/repository/github/young-tw/yu)

## Installation

```bash
curl -sSL https://raw.githubusercontent.com/Young-TW/yu/main/script/install.sh | bash
```

### Build from source

#### Build

Please make sure you have installed `rust` and `cargo`.

```bash
git clone https://github.com/Young-TW/yu.git
cd yu
cargo build --release
```

#### Install

```bash
cargo install --path .
```

please make sure the `~/.cargo/bin` is in your `PATH`.

## Usage

install package:

```bash
yu install <package>
```

uninstall package:

```bash
yu uninstall <package>
```

upgrade package:

```bash
yu
```

or

```bash
yu upgrade
```

package name is the same as the package name in the package manager of the system.
