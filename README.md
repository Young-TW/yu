# yu - a package manager wrapper for all unix-like systems

`yu` allows users to use the same command to install, uninstall, and upgrade packages on all unix-like systems.

[![CodeFactor](https://www.codefactor.io/repository/github/young-tw/yu/badge)](https://www.codefactor.io/repository/github/young-tw/yu)

## Installation

```bash
curl -sSL https://raw.githubusercontent.com/Young-TW/yu/main/script/install.sh | bash
```

## Supported package managers

- [x] `apt`
- [x] `dnf`
- [x] `pacman`
- [x] `yum`
- [x] `homebrew`
- [x] `zypper`
- [x] `portage`
- [x] `apk`

### Build from source manually

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

upgrade all packages:

```bash
yu
```

or

```bash
yu upgrade
```

package name is the same as the package name in the package manager of the system.

### Flags

- `-h`, `--help`: Show help message.
- `-v`, `--version`: Show version.
- `-V`, `--verbose`: Show verbose output.
- `-S`, `--silent`: Show no output(only error output).
